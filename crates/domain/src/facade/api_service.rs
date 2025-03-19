use crate::model::NotifyMessage;
use crate::model::endpoint::{
    EndpointCreateModel, EndpointReadModel, EndpointReadPreviewModel,
};
use crate::model::persistence::PersistenceErrorModel;
use crate::model::webhook::{WebhookRequestModel, WebhookRequestPreviewModel};
use crate::ports::notifier::NotifierPort;
use crate::ports::persistence::PersistencePort;
use dyn_clone::DynClone;

dyn_clone::clone_trait_object!(ApiServiceInterface);

#[async_trait::async_trait]
pub trait ApiServiceInterface: Send + Sync + DynClone {
    async fn create_endpoint(
        &self,
        endpoint: EndpointCreateModel,
    ) -> Result<EndpointReadModel, PersistenceErrorModel>;
    async fn create_random_endpoint(&self) -> Result<EndpointReadModel, PersistenceErrorModel>;
    async fn get_endpoints(&self) -> Vec<EndpointReadPreviewModel>;
    async fn get_endpoint(
        &self,
        url: String,
    ) -> Result<Option<EndpointReadModel>, PersistenceErrorModel>;
    async fn get_requests(&self, endpoint_id: i32) -> Vec<WebhookRequestPreviewModel>;
    async fn get_request_by_id(
        &self,
        request_id: i32,
    ) -> Result<Option<WebhookRequestModel>, PersistenceErrorModel>;
}

#[derive(Clone)]
pub struct ApiService<P, N>
where
    P: PersistencePort,
    N: NotifierPort,
{
    persistence: P,
    notifier: N,
}

impl<P, N> ApiService<P, N>
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
impl<P, N> ApiServiceInterface for ApiService<P, N>
where
    P: PersistencePort + 'static + Clone,
    N: NotifierPort + 'static + Clone,
{
    async fn create_endpoint(
        &self,
        endpoint: EndpointCreateModel,
    ) -> Result<EndpointReadModel, PersistenceErrorModel> {
        let endpoint = self.persistence.save_endpoint(endpoint).await?;
        let msg = NotifyMessage::EndpointAdded(endpoint.clone());
        self.notifier.notify(msg).await;
        Ok(endpoint)
    }

    async fn create_random_endpoint(&self) -> Result<EndpointReadModel, PersistenceErrorModel> {
        //TODO
        unimplemented!()
        /*
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
         */
    }

    async fn get_endpoints(&self) -> Vec<EndpointReadPreviewModel> {
        self.persistence.get_endpoints().await
    }

    async fn get_endpoint(
        &self,
        url: String,
    ) -> Result<Option<EndpointReadModel>, PersistenceErrorModel> {
        self.persistence.get_endpoint(url).await
    }

    async fn get_requests(&self, endpoint_id: i32) -> Vec<WebhookRequestPreviewModel> {
        self.persistence.get_requests_by_endpoint(endpoint_id).await
    }

    async fn get_request_by_id(
        &self,
        request_id: i32,
    ) -> Result<Option<WebhookRequestModel>, PersistenceErrorModel> {
        self.persistence.get_request_by_id(request_id).await
    }
}
