use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub(crate) enum PublicEndpoint {
    Table,
    Id,
    Url,
    CreatedAt,
}
