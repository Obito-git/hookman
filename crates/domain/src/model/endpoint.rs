use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EndpointReadDto {
    pub id: i32,
    pub url: String,
    pub action: Option<EndpointAction>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EndpointCreateDto {
    pub url: String,
    pub action: Option<EndpointAction>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum EndpointAction {
    CustomResponse(CustomResponseSettings),
    None,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CustomResponseSettings {
    pub status_code: u16,
    pub body: String,
    pub headers: serde_json::Value,
}
