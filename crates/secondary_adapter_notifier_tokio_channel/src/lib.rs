use async_trait::async_trait;
use domain::model::NotifyMessage;
use domain::ports::notifier::NotifierPort;
use tokio::sync::mpsc::Sender;

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
