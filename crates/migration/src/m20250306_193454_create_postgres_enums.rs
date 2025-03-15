use crate::endpoint::EndpointAction;
use crate::extension::postgres::Type;
use crate::request::HttpMethod;
use crate::sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum(HttpMethod::Enum)
                        .values([
                            HttpMethod::Get,
                            HttpMethod::Post,
                            HttpMethod::Put,
                            HttpMethod::Delete,
                            HttpMethod::Patch,
                            HttpMethod::Head,
                            HttpMethod::Connect,
                            HttpMethod::Options,
                            HttpMethod::Trace,
                        ])
                        .to_owned(),
                )
                .await?;
            manager
                .create_type(
                    Type::create()
                        .as_enum(EndpointAction::Enum)
                        .values([EndpointAction::CustomResponse])
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name(HttpMethod::Enum).to_owned())
                .await?;
        }
        Ok(())
    }
}
