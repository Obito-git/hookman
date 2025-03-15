use domain::model::webhook::WebhookRequestPreview;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::model::endpoint::EndpointReadDto;

#[derive(Deserialize)]
pub struct EndpointCreateDto {
    pub url: String,
}

impl From<EndpointReadDto> for EndpointResponseDto {
    fn from(value: EndpointReadDto) -> Self {
        Self {
            url: value.url,
        }
    }
}

#[derive(Serialize)]
pub struct EndpointResponseDto {
    pub url: String,
}

#[derive(Serialize)]
pub struct Metadata {
    count: usize,
    paging: String,
}

#[derive(Serialize)]
pub struct ReadAllEndpointsResponseDto {
    metadata: Metadata,
    endpoints: Vec<EndpointReadDto>,
}

//TODO: implement the complete one
impl ReadAllEndpointsResponseDto {
    pub fn new(requests: Vec<EndpointReadDto>) -> Self {
        Self {
            metadata: Metadata {
                count: requests.len(),
                paging: "TODO".to_string(),
            },
            endpoints: requests,
        }
    }
}

#[derive(Serialize)]
pub struct ReadAllRequestsResponseDto {
    metadata: Metadata,
    requests: Vec<WebhookRequestPreview>,
}

//TODO: implement the complete one
impl ReadAllRequestsResponseDto {
    pub fn new(requests: Vec<WebhookRequestPreview>) -> Self {
        Self {
            metadata: Metadata {
                count: requests.len(),
                paging: "TODO".to_string(),
            },
            requests,
        }
    }
}
