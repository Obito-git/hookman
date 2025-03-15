pub mod endpoint;
pub mod webhook;
pub mod persistence;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NotifyMessage {
    pub method: String,
    pub date: String,
    pub host: String,
    pub id: i32,
}



