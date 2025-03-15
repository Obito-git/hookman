use crate::endpoint::PublicEndpoint;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PublicEndpoint::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PublicEndpoint::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // todo: unique by userId + url for private endpoints
                    // todo: uuid for public endpoints bcs no users and no custom urls
                    .col(ColumnDef::new(PublicEndpoint::Url).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(PublicEndpoint::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
