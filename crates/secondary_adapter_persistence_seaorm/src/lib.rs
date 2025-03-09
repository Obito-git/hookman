use crate::mapper::map_db_err;
use async_trait::async_trait;
use domain::models::{PersistenceError, PersistenceType, PublicEndpoint, WebhookRequest};
use domain::ports::PersistencePort;
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::Error;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, RuntimeErr,
};

mod entity;
mod mapper;

const IN_MEMORY_SQLITE_URL: &'static str = "sqlite::memory:";

#[derive(Clone)]
pub struct SeaPersistence {
    pool: DatabaseConnection,
}

impl SeaPersistence {
    pub async fn new(persistence_type: PersistenceType) -> impl PersistencePort {
        let connection_url = match persistence_type {
            PersistenceType::InMemory => IN_MEMORY_SQLITE_URL.to_string(),
            PersistenceType::Postgres(config) => config.to_connection_string(),
            PersistenceType::SQLiteFile(config) => config.to_connection_string(),
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
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError> {
        entity::public_endpoint::Entity::find()
            .filter(entity::public_endpoint::Column::Url.eq(url))
            .one(&self.pool)
            .await
            .map(|model_opt| model_opt.map(|m| m.into()))
            .map_err(|e| map_db_err(e))
    }

    async fn save_endpoint(&self, url: Uuid) -> Result<PublicEndpoint, PersistenceError> {
        let new_endpoint = entity::public_endpoint::ActiveModel {
            id: Default::default(),
            url: ActiveValue::Set(url),
            created_at: Default::default(),
        };
        let res = entity::public_endpoint::Entity::insert(new_endpoint)
            .exec_with_returning(&self.pool)
            .await;
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
    }

    async fn get_endpoints(&self) -> Vec<PublicEndpoint> {
        entity::public_endpoint::Entity::find()
            .all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|model| model.into())
            .collect()
    }

    async fn save_request(&self, endpoint: PublicEndpoint, request: WebhookRequest) {
        let req = entity::public_request::ActiveModel {
            id: Default::default(),
            endpoint_id: ActiveValue::Set(endpoint.id),
            body: ActiveValue::Set(request.body),
            headers: ActiveValue::Set(request.headers),
            http_method: ActiveValue::Set(request.http_method.into()),
            timestamp: ActiveValue::Set(request.timestamp.naive_utc()),
            host: ActiveValue::Set(request.host),
            query_params: ActiveValue::Set(request.query_params),
        };
        entity::public_request::Entity::insert(req)
            .exec(&self.pool)
            .await
            .unwrap();
    }

    async fn get_requests_by_id(&self, endpoint_id: i32) -> Vec<WebhookRequest> {
        entity::public_request::Entity::find()
            .filter(entity::public_request::Column::EndpointId.eq(endpoint_id))
            .all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|model| model.into())
            .collect()
    }

    async fn get_requests(&self) -> Vec<WebhookRequest> {
        todo!()
    }
}
