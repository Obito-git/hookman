#[derive(Debug)]
pub enum PersistenceError {
    ResourceAlreadyExists,
    UnhandledError,
}

pub struct PostgresConfiguration {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

pub struct SQLiteConfiguration {
    pub database_name: String,
    pub folder_path: String,
}

impl SQLiteConfiguration {
    pub fn to_connection_string(&self) -> String {
        format!(
            "sqlite://{}/{}.sqlite?mode=rwc",
            self.folder_path, self.database_name
        )
    }
}

impl PostgresConfiguration {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

pub enum PersistenceType {
    InMemory,
    SQLiteFile(SQLiteConfiguration),
    Postgres(PostgresConfiguration),
}
