use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
#[sea_orm(enum_name = "http_method")]
pub(crate) enum HttpMethod {
    #[sea_orm(iden = "http_method")]
    Enum,
    #[sea_orm(iden = "get")]
    Get,
    #[sea_orm(iden = "post")]
    Post,
    #[sea_orm(iden = "put")]
    Put,
    #[sea_orm(iden = "delete")]
    Delete,
    #[sea_orm(iden = "patch")]
    Patch,
    #[sea_orm(iden = "head")]
    Head,
    #[sea_orm(iden = "connect")]
    Connect,
    #[sea_orm(iden = "options")]
    Options,
    #[sea_orm(iden = "trace")]
    Trace,
}

#[derive(DeriveIden)]
pub(crate) enum PublicRequest {
    Table,
    Id,
    EndpointId,
    HttpMethod,
    QueryParams,
    Body,
    Headers,
    Timestamp,
    Host,
}
