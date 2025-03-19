pub mod endpoint;
pub mod webhook;
pub mod persistence;

use crate::model::endpoint::EndpointReadModel;
use crate::model::webhook::WebhookRequestPreviewModel;
use serde::Serialize;
/*
#[derive(Debug, Clone, Serialize)]
pub struct NotifyMessage {
    pub method: String,
    pub date: String,
    pub host: String,
    pub id: i32,
}
 */

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum NotifyMessage {
    #[serde(rename_all = "camelCase")]
    EndpointAdded(EndpointReadModel),
    #[serde(rename_all = "camelCase")]
    EndpointRemoved(EndpointReadModel),
    #[serde(rename_all = "camelCase")]
    RequestAdded(WebhookRequestPreviewModel)
}



