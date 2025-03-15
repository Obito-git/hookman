use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub(crate) enum PublicEndpoint {
    Table,
    Id,
    Url,
    CreatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "endpoint_action")]
pub(crate) enum EndpointAction {
    #[sea_orm(iden = "endpoint_action")]
    Enum,
    #[sea_orm(iden = "custom_response")]
    CustomResponse,
}

#[derive(DeriveIden)]
pub(crate) enum EndpointActionSettings {
    Table,
    Id,
    EndpointId,
    ActionType,
    Payload,
}

