use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use domain::ports::NotifierPort;

#[derive(Clone)]
pub struct TokioChannelNotifier {
    tx: Sender<String>,
}

impl TokioChannelNotifier {
    pub fn new(sender: Sender<String>) -> Self {
        Self { tx: sender }
    }
}

#[async_trait]
impl NotifierPort for TokioChannelNotifier {
    async fn notify(&self, message: &str) {
        self.tx.send(message.to_string()).await.unwrap()
    }
}
