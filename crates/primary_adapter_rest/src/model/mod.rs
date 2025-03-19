use domain::model::webhook::WebhookRequestPreviewModel;
use serde::{Deserialize, Serialize};
use domain::model::endpoint::{EndpointReadModel, EndpointReadPreviewModel};

#[derive(Deserialize)]
pub struct EndpointCreateDto {
    pub url: String,
}

impl From<EndpointReadModel> for EndpointResponseDto {
    fn from(value: EndpointReadModel) -> Self {
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
    endpoints: Vec<EndpointReadPreviewModel>,
}

//TODO: implement the complete one
impl ReadAllEndpointsResponseDto {
    pub fn new(requests: Vec<EndpointReadPreviewModel>) -> Self {
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
    requests: Vec<WebhookRequestPreviewModel>,
}

//TODO: implement the complete one
impl ReadAllRequestsResponseDto {
    pub fn new(requests: Vec<WebhookRequestPreviewModel>) -> Self {
        Self {
            metadata: Metadata {
                count: requests.len(),
                paging: "TODO".to_string(),
            },
            requests,
        }
    }
}
