use crate::entity;
use domain::model::endpoint::EndpointReadPreviewModel;
use domain::model::persistence::PersistenceErrorModel;
use domain::model::webhook::{HttpMethodModel, WebhookRequestModel, WebhookRequestPreviewModel};
use entity::sea_orm_active_enums::HttpMethod as SeaOrmHttpMethod;
use sea_orm::DbErr;

//TODO: or wrap struct?
pub(crate) fn map_db_err(db_err: DbErr) -> PersistenceErrorModel {
    match db_err {
        _ => PersistenceErrorModel::UnhandledError,
    }
}

impl From<entity::public_endpoint::Model> for EndpointReadPreviewModel {
    fn from(value: entity::public_endpoint::Model) -> Self {
        EndpointReadPreviewModel {
            id: value.id,
            url: value.url,
        }
    }
}

impl From<entity::public_request::Model> for WebhookRequestModel {
    fn from(value: entity::public_request::Model) -> Self {
        WebhookRequestModel {
            headers: value.headers,
            query_params: value.query_params,
            body: value.body,
            timestamp: value.timestamp.and_utc(),
            host: value.host, //TODO
            method: value.http_method.into(),
        }
    }
}

impl From<entity::public_request::Model> for WebhookRequestPreviewModel {
    fn from(value: entity::public_request::Model) -> Self {
        WebhookRequestPreviewModel {
            timestamp: value.timestamp.and_utc(),
            host: value.host, //TODO
            method: value.http_method.into(),
            id: value.id,
            endpoint_id: value.endpoint_id,
        }
    }
}

impl From<HttpMethodModel> for SeaOrmHttpMethod {
    fn from(value: HttpMethodModel) -> Self {
        match value {
            HttpMethodModel::Get => SeaOrmHttpMethod::Get,
            HttpMethodModel::Post => SeaOrmHttpMethod::Post,
            HttpMethodModel::Put => SeaOrmHttpMethod::Put,
            HttpMethodModel::Delete => SeaOrmHttpMethod::Delete,
            HttpMethodModel::Patch => SeaOrmHttpMethod::Patch,
            HttpMethodModel::Head => SeaOrmHttpMethod::Head,
            HttpMethodModel::Connect => SeaOrmHttpMethod::Connect,
            HttpMethodModel::Options => SeaOrmHttpMethod::Options,
            HttpMethodModel::Trace => SeaOrmHttpMethod::Trace,
        }
    }
}

impl From<SeaOrmHttpMethod> for HttpMethodModel {
    fn from(value: SeaOrmHttpMethod) -> Self {
        match value {
            SeaOrmHttpMethod::Get => HttpMethodModel::Get,
            SeaOrmHttpMethod::Post => HttpMethodModel::Post,
            SeaOrmHttpMethod::Put => HttpMethodModel::Put,
            SeaOrmHttpMethod::Delete => HttpMethodModel::Delete,
            SeaOrmHttpMethod::Patch => HttpMethodModel::Patch,
            SeaOrmHttpMethod::Head => HttpMethodModel::Head,
            SeaOrmHttpMethod::Connect => HttpMethodModel::Connect,
            SeaOrmHttpMethod::Options => HttpMethodModel::Options,
            SeaOrmHttpMethod::Trace => HttpMethodModel::Trace,
        }
    }
}
