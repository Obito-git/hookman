#[derive(Debug)]
pub enum PersistenceErrorModel {
    ResourceAlreadyExists,
    UnhandledError,
}

pub struct PostgresConfigurationModel {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

pub struct SQLiteConfigurationModel {
    pub database_name: String,
    pub folder_path: String,
}

impl SQLiteConfigurationModel {
    pub fn to_connection_string(&self) -> String {
        format!(
            "sqlite://{}/{}.sqlite?mode=rwc",
            self.folder_path, self.database_name
        )
    }
}

impl PostgresConfigurationModel {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

pub enum PersistenceTypeModel {
    InMemory,
    SQLiteFile(SQLiteConfigurationModel),
    Postgres(PostgresConfigurationModel),
}
