use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use domain::models::NotifyMessage;
use domain::ports::NotifierPort;

#[derive(Clone)]
pub struct TokioChannelNotifier {
    tx: Sender<NotifyMessage>,
}

impl TokioChannelNotifier {
    pub fn new(sender: Sender<NotifyMessage>) -> Self {
        Self { tx: sender }
    }
}

#[async_trait]
impl NotifierPort for TokioChannelNotifier {
    async fn notify(&self, message: NotifyMessage) {
        self.tx.send(message).await.unwrap()
    }
}
