use async_trait::async_trait;
use crate::model::NotifyMessage;

#[async_trait]
pub trait NotifierPort: Send + Sync + Clone {
    async fn notify(&self, message: NotifyMessage);
}