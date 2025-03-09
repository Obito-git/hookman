pub use sea_orm_migration::prelude::*;

mod endpoint;
mod m20250306_193454_create_postgres_enum_for_http_method;
mod m20250306_193522_create_public_endpoint_table;
mod m20250306_193530_create_public_requests_table;
mod request;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250306_193454_create_postgres_enum_for_http_method::Migration),
            Box::new(m20250306_193522_create_public_endpoint_table::Migration),
            Box::new(m20250306_193530_create_public_requests_table::Migration),
        ]
    }
}
