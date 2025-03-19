use crate::model::NotifyMessage;
use crate::model::endpoint::EndpointReadModel;
use crate::model::persistence::PersistenceErrorModel;
use crate::model::webhook::{WebhookRequestModel, WebhookRequestPreviewModel};
use crate::ports::notifier::NotifierPort;
use crate::ports::persistence::PersistencePort;
use dyn_clone::DynClone;

dyn_clone::clone_trait_object!(WebhookServiceInterface);

#[async_trait::async_trait]
pub trait WebhookServiceInterface: Send + Sync + DynClone {
    async fn process_request(
        &mut self,
        endpoint: EndpointReadModel,
        request: WebhookRequestModel,
    );
    async fn get_endpoint(
        &self,
        url: String,
    ) -> Result<Option<EndpointReadModel>, PersistenceErrorModel>;
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
    async fn process_request(
        &mut self,
        endpoint: EndpointReadModel,
        request: WebhookRequestModel,
    ) {
        let http_method = request.method.clone();
        let timestamp = request.timestamp;
        let endpoint_id = endpoint.id;
        let host = request.host.clone();
        let id = self
            .persistence
            .save_request(endpoint, request)
            .await
            .unwrap();
        let msg = NotifyMessage::RequestAdded(WebhookRequestPreviewModel {
            timestamp,
            host,
            method: http_method,
            id,
            endpoint_id,
        });
        self.notifier.notify(msg).await;
    }

    async fn get_endpoint(
        &self,
        url: String,
    ) -> Result<Option<EndpointReadModel>, PersistenceErrorModel> {
        self.persistence.get_endpoint(url).await
    }
}
