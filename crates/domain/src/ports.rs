use crate::models::{NotifyMessage, PersistenceError, PublicEndpoint, WebhookRequest, WebhookRequestPreview};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PersistencePort: Send + Sync + Clone {
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError>;
    async fn save_endpoint(&self, url: Uuid) -> Result<PublicEndpoint, PersistenceError>;
    async fn get_endpoints(&self) -> Vec<PublicEndpoint>;
    async fn save_request(&self, endpoint: PublicEndpoint, request: WebhookRequest) -> Result<i32, PersistenceError>;
    async fn get_requests_by_endpoint(&self, id: i32) -> Vec<WebhookRequestPreview>;
    async fn get_request_by_id(&self, id: i32) -> Result<Option<WebhookRequest>, PersistenceError>;
    async fn get_requests(&self) -> Vec<WebhookRequest>;
}

#[async_trait]
pub trait NotifierPort: Send + Sync + Clone {
    async fn notify(&self, message: NotifyMessage);
}

pub trait CrudPort {
    fn create_endpoint(&mut self, endpoint: PublicEndpoint);
    fn list_endpoints(&self) -> Vec<PublicEndpoint>;
}
