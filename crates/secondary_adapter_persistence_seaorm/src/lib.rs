use crate::mapper::map_db_err;
use async_trait::async_trait;
use domain::model::endpoint::{
    CustomResponseModel, EndpointActionModel, EndpointCreateModel, EndpointReadModel,
    EndpointReadPreviewModel,
};
use domain::model::persistence::{PersistenceErrorModel, PersistenceTypeModel};
use domain::model::webhook::{WebhookRequestModel, WebhookRequestPreviewModel};
use domain::ports::persistence::PersistencePort;
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder, TransactionTrait,
};

mod entity;
mod mapper;

const IN_MEMORY_SQLITE_URL: &'static str = "sqlite::memory:";

#[derive(Clone)]
pub struct SeaPersistence {
    pool: DatabaseConnection,
}

impl SeaPersistence {
    pub async fn new(persistence_type: PersistenceTypeModel) -> impl PersistencePort {
        let connection_url = match persistence_type {
            PersistenceTypeModel::InMemory => IN_MEMORY_SQLITE_URL.to_string(),
            PersistenceTypeModel::Postgres(config) => config.to_connection_string(),
            PersistenceTypeModel::SQLiteFile(config) => config.to_connection_string(),
        };

        info!("Connection URL: {connection_url}");

        let mut opt = ConnectOptions::new(connection_url);
        opt.min_connections(32).max_connections(64);
        let pool = Database::connect(opt)
            .await
            .expect("Can't establish connection");

        Migrator::up(&pool, None).await.unwrap();

        SeaPersistence { pool }
    }
}

#[async_trait]
impl PersistencePort for SeaPersistence {
    async fn get_endpoint(
        &self,
        url: String,
    ) -> Result<Option<EndpointReadModel>, PersistenceErrorModel> {
        let mod_opt = entity::public_endpoint::Entity::find()
            .filter(entity::public_endpoint::Column::Url.eq(url))
            .one(&self.pool)
            .await
            .map_err(|e| map_db_err(e))?;
        if let Some(endpoint) = mod_opt {
            let action = entity::endpoint_action_settings::Entity::find()
                .filter(entity::endpoint_action_settings::Column::EndpointId.eq(endpoint.id))
                .one(&self.pool)
                .await
                .map_err(|e| map_db_err(e))
                .map(|action_opt| {
                    action_opt.map(|action| {
                        let payload = action.payload.unwrap();
                        let settings: CustomResponseModel =
                            serde_json::from_value(payload).unwrap();
                        EndpointActionModel::CustomResponse(settings)
                    })
                })?;
            Ok(Some(EndpointReadModel {
                id: endpoint.id,
                url: endpoint.url,
                action,
            }))
        } else {
            Ok(mod_opt.map(|e| EndpointReadModel {
                id: e.id,
                url: e.url,
                action: None,
            }))
        }
    }

    async fn save_endpoint(
        &self,
        endpoint: EndpointCreateModel,
    ) -> Result<EndpointReadModel, PersistenceErrorModel> {
        self.pool
            .transaction::<_, EndpointReadModel, DbErr>(|txn| {
                Box::pin(async move {
                    let endpoint_model = entity::public_endpoint::ActiveModel {
                        id: Default::default(),
                        url: ActiveValue::Set(endpoint.url),
                        created_at: Default::default(),
                    };
                    let created_endpoint = entity::public_endpoint::Entity::insert(endpoint_model)
                        .exec_with_returning(txn)
                        .await?;

                    // TODO: put in separate method
                    if endpoint.action.is_none() {
                        return Ok(EndpointReadModel {
                            id: created_endpoint.id,
                            url: created_endpoint.url,
                            action: None,
                        });
                    }
                    let action_type = endpoint
                        .action
                        .as_ref()
                        .map(|v| match v {
                            EndpointActionModel::CustomResponse(_) => {
                                Some(entity::sea_orm_active_enums::EndpointAction::CustomResponse)
                            }
                            EndpointActionModel::None => None,
                        })
                        .flatten();
                    let payload = endpoint.action.map(|v| match v {
                        EndpointActionModel::CustomResponse(settings) => {
                            serde_json::to_value(settings).unwrap()
                        }
                        EndpointActionModel::None => serde_json::Value::Null,
                    });
                    let action_model = entity::endpoint_action_settings::ActiveModel {
                        id: Default::default(),
                        endpoint_id: ActiveValue::Set(created_endpoint.id),
                        action_type: ActiveValue::Set(action_type),
                        payload: ActiveValue::Set(payload),
                    };
                    let created_actions =
                        entity::endpoint_action_settings::Entity::insert(action_model)
                            .exec_with_returning(txn)
                            .await?;
                    Ok(EndpointReadModel {
                        id: created_endpoint.id,
                        url: created_endpoint.url,
                        action: if let Some(action) = created_actions.action_type {
                            match action {
                                entity::sea_orm_active_enums::EndpointAction::CustomResponse => {
                                    let payload = created_actions.payload.unwrap();
                                    let settings: CustomResponseModel =
                                        serde_json::from_value(payload).unwrap();
                                    Some(EndpointActionModel::CustomResponse(settings))
                                }
                            }
                        } else {
                            None
                        },
                    })
                })
            })
            .await
            .map_err(|_| PersistenceErrorModel::UnhandledError)

        /*
        println!("{res:?}");
        match res {
            Ok(model) => Ok(model.into()),
            Err(error) => {
                println!("SQL ERR: {:?}", error.sql_err());

                if let DbErr::Query(RuntimeErr::SqlxError(Error::Database(concrete))) = error {
                    println!("Concrete: {}, code: {:?}", concrete, concrete.code());
                }
                Err(PersistenceError::UnhandledError)
            }
        }
         */
    }

    async fn get_endpoints(&self) -> Vec<EndpointReadPreviewModel> {
        entity::public_endpoint::Entity::find()
            .all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect()
    }

    async fn save_request(
        &self,
        endpoint: EndpointReadModel,
        request: WebhookRequestModel,
    ) -> Result<i32, PersistenceErrorModel> {
        let req = entity::public_request::ActiveModel {
            id: Default::default(),
            endpoint_id: ActiveValue::Set(endpoint.id),
            body: ActiveValue::Set(request.body),
            headers: ActiveValue::Set(request.headers),
            http_method: ActiveValue::Set(request.method.into()),
            timestamp: ActiveValue::Set(request.timestamp.naive_utc()),
            host: ActiveValue::Set(request.host),
            query_params: ActiveValue::Set(request.query_params),
        };
        entity::public_request::Entity::insert(req)
            .exec_with_returning(&self.pool)
            .await
            .map(|n| n.id)
            .map_err(|e| map_db_err(e))
    }

    async fn get_requests_by_endpoint(&self, endpoint_id: i32) -> Vec<WebhookRequestPreviewModel> {
        entity::public_request::Entity::find()
            .filter(entity::public_request::Column::EndpointId.eq(endpoint_id))
            .order_by_desc(entity::public_request::Column::Timestamp)
            .all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect()
    }

    async fn get_request_by_id(&self, id: i32) -> Result<Option<WebhookRequestModel>, PersistenceErrorModel> {
        let model_opt = entity::public_request::Entity::find()
            .filter(entity::public_request::Column::Id.eq(id))
            .one(&self.pool)
            .await
            .map_err(map_db_err)?;
        Ok(model_opt.map(Into::into))
    }

    async fn get_requests(&self) -> Vec<WebhookRequestModel> {
        todo!()
    }
}
