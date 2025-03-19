mod api;

use api::arg_config::AppConfig;
use clap::Parser;
use domain::facade::api_service::ApiService;
use domain::facade::webhook_service::WebhookService;
use secondary_adapter_notifier_tokio_channel::TokioChannelNotifier;
use secondary_adapter_persistence_seaorm::SeaPersistence;
use tokio::sync::mpsc;
use tracing::log::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::parse();
    info!("Starting the server with configuration:\n{config}");

    let persistence_adapter = SeaPersistence::new(config.persistence_type()).await;
    let (tx, _rx) = mpsc::channel(32);
    let notifier_adapter = TokioChannelNotifier::new(tx);
    let webhook_service =
        WebhookService::new(persistence_adapter.clone(), notifier_adapter.clone());
    let api_service = ApiService::new(persistence_adapter, notifier_adapter.clone());

    let webhook_server_join = primary_adapter_webhook::start_server(
        &format!("{}:{}", config.host, config.webhook_port),
        Box::new(webhook_service),
    );
    let api_server_join = primary_adapter_rest::start_server(
        &format!("{}:{}", config.host, config.ui_api_port),
        Box::new(api_service),
    );

    webhook_server_join.join().expect("Thread panicked");
    api_server_join.join().expect("Thread panicked")
}
