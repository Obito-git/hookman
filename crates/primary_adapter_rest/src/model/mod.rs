use domain::models::{PublicEndpoint, WebhookRequest, WebhookRequestPreview};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct EndpointCreateDto {
    pub endpoint: Uuid,
}

impl From<PublicEndpoint> for EndpointResponseDto {
    fn from(value: PublicEndpoint) -> Self {
        Self {
            endpoint: value.uri,
        }
    }
}

#[derive(Serialize)]
pub struct EndpointResponseDto {
    pub endpoint: Uuid,
}

#[derive(Serialize)]
pub struct Metadata {
    count: usize,
    paging: String,
}

#[derive(Serialize)]
pub struct ReadAllEndpointsResponseDto {
    metadata: Metadata,
    endpoints: Vec<PublicEndpoint>,
}

//TODO: implement the complete one
impl ReadAllEndpointsResponseDto {
    pub fn new(requests: Vec<PublicEndpoint>) -> Self {
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
