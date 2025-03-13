use crate::entity;
use domain::models::{HttpMethod, PersistenceError, PublicEndpoint, WebhookRequest, WebhookRequestPreview};
use sea_orm::DbErr;

//TODO: or wrap struct?
pub(crate) fn map_db_err(db_err: DbErr) -> PersistenceError {
    match db_err {
        _ => PersistenceError::UnhandledError,
    }
}

impl From<entity::public_endpoint::Model> for PublicEndpoint {
    fn from(value: entity::public_endpoint::Model) -> Self {
        PublicEndpoint {
            id: value.id,
            uri: value.url,
        }
    }
}

impl From<entity::public_request::Model> for WebhookRequest {
    fn from(value: entity::public_request::Model) -> Self {
        WebhookRequest {
            headers: value.headers,
            query_params: value.query_params,
            body: value.body,
            timestamp: value.timestamp.and_utc(),
            host: value.host, //TODO
            http_method: value.http_method.into(),
        }
    }
}

impl From<entity::public_request::Model> for WebhookRequestPreview {
    fn from(value: entity::public_request::Model) -> Self {
        WebhookRequestPreview {
            timestamp: value.timestamp.and_utc(),
            host: value.host, //TODO
            http_method: value.http_method.into(),
            id: value.id,
        }
    }
}

impl From<HttpMethod> for entity::sea_orm_active_enums::HttpMethod {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => entity::sea_orm_active_enums::HttpMethod::Get,
            HttpMethod::Post => entity::sea_orm_active_enums::HttpMethod::Post,
            HttpMethod::Put => entity::sea_orm_active_enums::HttpMethod::Put,
            HttpMethod::Delete => entity::sea_orm_active_enums::HttpMethod::Delete,
            HttpMethod::Patch => entity::sea_orm_active_enums::HttpMethod::Patch,
            HttpMethod::Head => entity::sea_orm_active_enums::HttpMethod::Head,
            HttpMethod::Connect => entity::sea_orm_active_enums::HttpMethod::Connect,
            HttpMethod::Options => entity::sea_orm_active_enums::HttpMethod::Options,
            HttpMethod::Trace => entity::sea_orm_active_enums::HttpMethod::Trace,
        }
    }
}

impl From<entity::sea_orm_active_enums::HttpMethod> for HttpMethod {
    fn from(value: entity::sea_orm_active_enums::HttpMethod) -> Self {
        match value {
            entity::sea_orm_active_enums::HttpMethod::Get => HttpMethod::Get,
            entity::sea_orm_active_enums::HttpMethod::Post => HttpMethod::Post,
            entity::sea_orm_active_enums::HttpMethod::Put => HttpMethod::Put,
            entity::sea_orm_active_enums::HttpMethod::Delete => HttpMethod::Delete,
            entity::sea_orm_active_enums::HttpMethod::Patch => HttpMethod::Patch,
            entity::sea_orm_active_enums::HttpMethod::Head => HttpMethod::Head,
            entity::sea_orm_active_enums::HttpMethod::Connect => HttpMethod::Connect,
            entity::sea_orm_active_enums::HttpMethod::Options => HttpMethod::Options,
            entity::sea_orm_active_enums::HttpMethod::Trace => HttpMethod::Trace,
        }
    }
}
