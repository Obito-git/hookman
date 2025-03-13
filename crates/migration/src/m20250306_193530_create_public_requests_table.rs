use crate::endpoint::PublicEndpoint;
use crate::request::{HttpMethod, PublicRequest};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    ///TODO: add constraints
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PublicRequest::Table)
                    .if_not_exists()
                    .col(
                        //TODO: uuid PK?
                        ColumnDef::new(PublicRequest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PublicRequest::EndpointId).integer().not_null())
                    .col(ColumnDef::new(PublicRequest::Body).text())
                    .col(ColumnDef::new(PublicRequest::Host).text().not_null())
                    .col(ColumnDef::new(PublicRequest::Headers).json_binary().not_null())
                    .col(ColumnDef::new(PublicRequest::QueryParams).json_binary())
                    .col(
                        ColumnDef::new(PublicRequest::HttpMethod)
                            .custom(HttpMethod::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PublicRequest::Timestamp)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    // Create the foreign key constraint.
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_received_requests_endpoint_id")
                            .from(PublicRequest::Table, PublicRequest::EndpointId)
                            .to(PublicEndpoint::Table, PublicEndpoint::Id),
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
