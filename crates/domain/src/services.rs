use crate::models::{PersistenceError, PublicEndpoint, WebhookRequest};
use crate::ports::{NotifierPort, PersistencePort};
use dyn_clone::DynClone;
use uuid::Uuid;

dyn_clone::clone_trait_object!(ApiServiceInterface);

#[async_trait::async_trait]
pub trait WebhookServiceInterface: Send + Sync + DynClone {
    async fn process_request(&mut self, endpoint: PublicEndpoint, request: WebhookRequest);
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError>;
}

#[derive(Clone)]
pub struct WebhookService<P, N>
where
    P: PersistencePort,
    N: NotifierPort,
{
    persistence: P,
    notifier: N,
}

impl<P, N> WebhookService<P, N>
where
    P: PersistencePort,
    N: NotifierPort,
{
    pub fn new(persistence: P, notifier: N) -> Self {
        Self {
            persistence,
            notifier,
        }
    }
}

#[async_trait::async_trait]
impl<P, N> WebhookServiceInterface for WebhookService<P, N>
where
    P: PersistencePort + 'static + Clone,
    N: NotifierPort + 'static + Clone,
{
    async fn process_request(&mut self, endpoint: PublicEndpoint, request: WebhookRequest) {
        let msg = format!("Processed request: {:?}", request);
        self.persistence.save_request(endpoint, request).await;
        self.notifier.notify(msg.as_str()).await;
    }

    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError> {
        self.persistence.get_endpoint(url).await
    }
}

dyn_clone::clone_trait_object!(WebhookServiceInterface);

#[async_trait::async_trait]
pub trait ApiServiceInterface: Send + Sync + DynClone {
    async fn create_endpoint(&self, endpoint: Uuid) -> Result<PublicEndpoint, PersistenceError>;
    async fn create_random_endpoint(&self) -> Result<PublicEndpoint, PersistenceError>;
    async fn get_endpoints(&self) -> Vec<PublicEndpoint>;
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError>;
    async fn get_requests(&self, endpoint_id: i32) -> Vec<WebhookRequest>;
}

#[derive(Clone)]
pub struct ApiService<P>
where
    P: PersistencePort,
{
    persistence: P,
}

impl<P> ApiService<P>
where
    P: PersistencePort,
{
    pub fn new(persistence: P) -> Self {
        Self { persistence }
    }
}

#[async_trait::async_trait]
impl<P> ApiServiceInterface for ApiService<P>
where
    P: PersistencePort + 'static + Clone,
{
    async fn create_endpoint(&self, endpoint: Uuid) -> Result<PublicEndpoint, PersistenceError> {
        self.persistence.save_endpoint(endpoint).await
    }

    async fn create_random_endpoint(&self) -> Result<PublicEndpoint, PersistenceError> {
        loop {
            let new_uuid = Uuid::new_v4();
            match self.persistence.save_endpoint(new_uuid).await {
                Ok(endpoint) => return Ok(endpoint),
                Err(PersistenceError::ResourceAlreadyExists) => {
                    continue;
                }
                Err(other_error) => return Err(other_error),
            }
        }
    }

    async fn get_endpoints(&self) -> Vec<PublicEndpoint> {
        self.persistence.get_endpoints().await
    }

    async fn get_endpoint(&self, url: Uuid) -> Result<Option<PublicEndpoint>, PersistenceError> {
        self.persistence.get_endpoint(url).await
    }

    async fn get_requests(&self, endpoint_id: i32) -> Vec<WebhookRequest> {
        self.persistence.get_requests_by_id(endpoint_id).await
    }
}

