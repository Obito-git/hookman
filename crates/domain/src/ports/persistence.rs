use async_trait::async_trait;
use crate::model::endpoint::{EndpointCreateModel, EndpointReadModel, EndpointReadPreviewModel};
use crate::model::persistence::PersistenceErrorModel;
use crate::model::webhook::{WebhookRequestModel, WebhookRequestPreviewModel};

#[async_trait]
pub trait PersistencePort: Send + Sync + Clone {
    async fn get_endpoint(&self, url: String) -> Result<Option<EndpointReadModel>, PersistenceErrorModel>;
    async fn save_endpoint(&self, endpoint: EndpointCreateModel) -> Result<EndpointReadModel, PersistenceErrorModel>;
    async fn get_endpoints(&self) -> Vec<EndpointReadPreviewModel>;
    async fn save_request(
        &self,
        endpoint: EndpointReadModel,
        request: WebhookRequestModel,
    ) -> Result<i32, PersistenceErrorModel>;
    async fn get_requests_by_endpoint(&self, id: i32) -> Vec<WebhookRequestPreviewModel>;
    async fn get_request_by_id(&self, id: i32) -> Result<Option<WebhookRequestModel>, PersistenceErrorModel>;
    async fn get_requests(&self) -> Vec<WebhookRequestModel>;
}