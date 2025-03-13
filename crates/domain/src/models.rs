use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct NotifyMessage {
    pub method: String,
    pub date: String,
    pub host: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Connect,
    Options,
    Trace,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Connect => write!(f, "CONNECT"),
            HttpMethod::Options => write!(f, "OPTIONS"),
            HttpMethod::Trace => write!(f, "TRACE"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookRequest {
    pub headers: serde_json::Value,
    pub query_params: Option<serde_json::Value>,
    pub body: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub host: String,
    pub http_method: HttpMethod,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookRequestPreview {
    pub timestamp: DateTime<Utc>,
    pub host: String,
    pub http_method: HttpMethod,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PublicEndpoint {
    pub id: i32,
    pub uri: Uuid,
}

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
