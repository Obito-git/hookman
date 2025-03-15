use crate::model::NotifyMessage;
use crate::model::persistence::PersistenceError;
use crate::model::webhook::{WebhookRequest, WebhookRequestPreview};
use crate::ports::{NotifierPort, PersistencePort};
use dyn_clone::DynClone;
use uuid::Uuid;
use crate::model::endpoint::EndpointReadDto;

dyn_clone::clone_trait_object!(ApiServiceInterface);

#[async_trait::async_trait]
pub trait WebhookServiceInterface: Send + Sync + DynClone {
    async fn process_request(&mut self, endpoint: EndpointReadDto, request: WebhookRequest);
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<EndpointReadDto>, PersistenceError>;
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
    async fn process_request(&mut self, endpoint: EndpointReadDto, request: WebhookRequest) {
        let method = request.http_method.to_string();
        let date = request.timestamp.to_string();
        let host = request.host.clone();
        let id = self
            .persistence
            .save_request(endpoint, request)
            .await
            .unwrap();
        let msg = NotifyMessage {
            method,
            date,
            host,
            id,
        };
        self.notifier.notify(msg).await;
    }

    async fn get_endpoint(&self, url: Uuid) -> Result<Option<EndpointReadDto>, PersistenceError> {
        self.persistence.get_endpoint(url).await
    }
}

dyn_clone::clone_trait_object!(WebhookServiceInterface);

#[async_trait::async_trait]
pub trait ApiServiceInterface: Send + Sync + DynClone {
    async fn create_endpoint(&self, url: String) -> Result<EndpointReadDto, PersistenceError>;
    async fn create_random_endpoint(&self) -> Result<EndpointReadDto, PersistenceError>;
    async fn get_endpoints(&self) -> Vec<EndpointReadDto>;
    async fn get_endpoint(&self, url: Uuid) -> Result<Option<EndpointReadDto>, PersistenceError>;
    async fn get_requests(&self, endpoint_id: i32) -> Vec<WebhookRequestPreview>;
    async fn get_request_by_id(
        &self,
        request_id: i32,
    ) -> Result<Option<WebhookRequest>, PersistenceError>;
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
    async fn create_endpoint(&self, url: String) -> Result<EndpointReadDto, PersistenceError> {
        self.persistence.save_endpoint(url).await
    }

    async fn create_random_endpoint(&self) -> Result<EndpointReadDto, PersistenceError> {
        loop {
            let new_uuid = Uuid::new_v4();
            match self.persistence.save_endpoint(new_uuid.to_string()).await {
                Ok(endpoint) => return Ok(endpoint),
                Err(PersistenceError::ResourceAlreadyExists) => {
                    continue;
                }
                Err(other_error) => return Err(other_error),
            }
        }
    }

    async fn get_endpoints(&self) -> Vec<EndpointReadDto> {
        self.persistence.get_endpoints().await
    }

    async fn get_endpoint(&self, url: Uuid) -> Result<Option<EndpointReadDto>, PersistenceError> {
        self.persistence.get_endpoint(url).await
    }

    async fn get_requests(&self, endpoint_id: i32) -> Vec<WebhookRequestPreview> {
        self.persistence.get_requests_by_endpoint(endpoint_id).await
    }

    async fn get_request_by_id(
        &self,
        request_id: i32,
    ) -> Result<Option<WebhookRequest>, PersistenceError> {
        self.persistence.get_request_by_id(request_id).await
    }
}
