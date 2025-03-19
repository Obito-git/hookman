use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EndpointReadPreviewModel {
    pub id: i32,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EndpointReadModel {
    pub id: i32,
    pub url: String,
    pub action: Option<EndpointActionModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndpointCreateModel {
    pub url: String,
    pub action: Option<EndpointActionModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum EndpointActionModel {
    CustomResponse(CustomResponseModel),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomResponseModel {
    pub status_code: i16,
    pub body: String,
    pub headers: HashMap<String, String>,
}
