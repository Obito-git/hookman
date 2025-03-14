pub use sea_orm_migration::prelude::*;

mod endpoint;
mod m20250306_193454_create_postgres_enums;
mod m20250306_193522_create_public_endpoint_table;
mod m20250306_193530_create_public_requests_table;
mod m20250314_045815_add_endpoint_actions;
mod request;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250306_193454_create_postgres_enums::Migration),
            Box::new(m20250306_193522_create_public_endpoint_table::Migration),
            Box::new(m20250306_193530_create_public_requests_table::Migration),
            Box::new(m20250314_045815_add_endpoint_actions::Migration),
        ]
    }
}
