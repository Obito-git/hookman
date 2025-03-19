use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethodModel {
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

impl Display for HttpMethodModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethodModel::Get => write!(f, "GET"),
            HttpMethodModel::Post => write!(f, "POST"),
            HttpMethodModel::Put => write!(f, "PUT"),
            HttpMethodModel::Delete => write!(f, "DELETE"),
            HttpMethodModel::Patch => write!(f, "PATCH"),
            HttpMethodModel::Head => write!(f, "HEAD"),
            HttpMethodModel::Connect => write!(f, "CONNECT"),
            HttpMethodModel::Options => write!(f, "OPTIONS"),
            HttpMethodModel::Trace => write!(f, "TRACE"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookRequestModel {
    pub headers: serde_json::Value,
    pub query_params: Option<serde_json::Value>,
    pub body: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub host: String,
    pub method: HttpMethodModel,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookRequestPreviewModel {
    pub timestamp: DateTime<Utc>,
    pub host: String,
    pub method: HttpMethodModel,
    pub id: i32,
    pub endpoint_id: i32,
}
