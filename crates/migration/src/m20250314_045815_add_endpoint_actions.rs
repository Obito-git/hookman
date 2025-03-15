#![allow(unused)]
use sea_orm_migration::{prelude::*, schema::*};
use crate::endpoint::{EndpointAction, EndpointActionSettings, PublicEndpoint};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EndpointActionSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EndpointActionSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EndpointActionSettings::EndpointId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EndpointActionSettings::ActionType)
                            .custom(EndpointAction::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EndpointActionSettings::Payload)
                            .json_binary()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-endpoint-action-endpoint")
                            .from(EndpointActionSettings::Table, EndpointActionSettings::EndpointId)
                            .to(PublicEndpoint::Table, PublicEndpoint::Id)
                    )
                    .index(
                        Index::create()
                            .name("idx-unique-endpoint_action")
                            .table(EndpointActionSettings::Table)
                            .col(EndpointActionSettings::EndpointId)
                            .col(EndpointActionSettings::ActionType)
                            .unique()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
