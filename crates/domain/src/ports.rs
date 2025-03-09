use crate::models::{PersistenceError, PublicEndpoint, WebhookRequest};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PersistencePort: Send + Sync + Clone {
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError>;
    async fn save_endpoint(&self, url: Uuid) -> Result<PublicEndpoint, PersistenceError>;
    async fn get_endpoints(&self) -> Vec<PublicEndpoint>;
    async fn save_request(&self, endpoint: PublicEndpoint, request: WebhookRequest);
    async fn get_requests_by_id(&self, id: i32) -> Vec<WebhookRequest>;
    async fn get_requests(&self) -> Vec<WebhookRequest>;
}

#[async_trait]
pub trait NotifierPort: Send + Sync + Clone {
    async fn notify(&self, message: &str);
}

pub trait CrudPort {
    fn create_endpoint(&mut self, endpoint: PublicEndpoint);
    fn list_endpoints(&self) -> Vec<PublicEndpoint>;
}
