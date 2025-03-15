use crate::model::NotifyMessage;
use crate::model::endpoint::{EndpointCreateDto, EndpointReadDto};
use crate::model::persistence::PersistenceError;
use crate::model::webhook::{WebhookRequest, WebhookRequestPreview};
use async_trait::async_trait;

#[async_trait]
pub trait PersistencePort: Send + Sync + Clone {
    async fn get_endpoint(&self, url: String) -> Result<Option<EndpointReadDto>, PersistenceError>;
    async fn save_endpoint(&self, url: String) -> Result<EndpointReadDto, PersistenceError>;
    async fn get_endpoints(&self) -> Vec<EndpointReadDto>;
    async fn save_request(
        &self,
        endpoint: EndpointReadDto,
        request: WebhookRequest,
    ) -> Result<i32, PersistenceError>;
    async fn get_requests_by_endpoint(&self, id: i32) -> Vec<WebhookRequestPreview>;
    async fn get_request_by_id(&self, id: i32) -> Result<Option<WebhookRequest>, PersistenceError>;
    async fn get_requests(&self) -> Vec<WebhookRequest>;
}

#[async_trait]
pub trait NotifierPort: Send + Sync + Clone {
    async fn notify(&self, message: NotifyMessage);
}

pub trait CrudPort {
    fn create_endpoint(&mut self, endpoint: EndpointCreateDto);
    fn list_endpoints(&self) -> Vec<EndpointReadDto>;
}
