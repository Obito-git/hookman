use tauri::{Emitter, Manager, State};
use tokio::sync::mpsc;
use tracing::log::info;
use domain::facade::api_service::{ApiService, ApiServiceInterface};
use domain::facade::webhook_service::WebhookService;
use secondary_adapter_notifier_tokio_channel::TokioChannelNotifier;
use secondary_adapter_persistence_seaorm::SeaPersistence;
use domain::model::endpoint::{CustomResponseModel, EndpointCreateModel, EndpointReadModel, EndpointReadPreviewModel};
use domain::model::endpoint::EndpointActionModel::CustomResponse;
use domain::model::persistence::{PersistenceTypeModel, PostgresConfigurationModel};
use domain::model::webhook::{WebhookRequestModel, WebhookRequestPreviewModel};

#[tauri::command]
async fn greet(name: String, state: State<'_, AppData>) -> Result<String,()> {
    let data = state.service.get_endpoint(name).await.unwrap().unwrap();
    let data2 = state.service.get_requests(data.id).await;
    Ok(format!("{:?}", data2))
}

#[tauri::command]
async fn get_endpoints(state: State<'_, AppData>) -> Result<Vec<EndpointReadPreviewModel>,()> {
    Ok(state.service.get_endpoints().await)
}

#[tauri::command]
async fn get_requests_by_endpoint_id(endpoint_id: i32, state: State<'_, AppData>) -> Result<Vec<WebhookRequestPreviewModel>,()> {
    Ok(state.service.get_requests(endpoint_id).await)
}

#[tauri::command]
async fn get_request(request_id: i32, state: State<'_, AppData>) -> Result<WebhookRequestModel,()> {
    Ok(state.service.get_request_by_id(request_id).await.unwrap().unwrap())
}

#[tauri::command]
async fn create_endpoint(endpoint: EndpointCreateModel, state: State<'_, AppData>) -> Result<EndpointReadModel,()> {
    Ok(state.service.create_endpoint(endpoint).await.unwrap())
}

struct AppData {
    pub service: Box<dyn ApiServiceInterface>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    //TODO: Write in file
    tracing_subscriber::fmt::init();

    info!("Starting the server with configuration:\n");

    //let persistence_adapter = SeaPersistence::new(PersistenceType::SQLiteFile(SQLiteConfiguration { database_name: "sqfile_tauri".to_string(), folder_path: ".".to_string() })).await;
    let postgres = PostgresConfigurationModel {
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        host: "localhost".to_string(),
        port: 5432,
        database: "postgres".to_string(),
    };
    let persistence_adapter = SeaPersistence::new(PersistenceTypeModel::Postgres(postgres)).await;

    let (tx, mut rx) = mpsc::channel(32);
    let notifier_adapter = TokioChannelNotifier::new(tx);

    let webhook_service = WebhookService::new(persistence_adapter.clone(), notifier_adapter.clone());
    let api_service = ApiService::new(persistence_adapter, notifier_adapter);

    let webhook_server_join = primary_adapter_webhook::start_server(
        &format!("{}:{}", "127.0.0.1", 4343),
        Box::new(webhook_service),
    );
    let api_server_join = primary_adapter_rest::start_server(
        &format!("{}:{}", "127.0.0.1", 4242),
        Box::new(api_service.clone()),
    );

    let kaka = EndpointCreateModel {
        url: "".to_string(),
        action: Some(CustomResponse( CustomResponseModel {
            status_code: 0,
            body: "".to_string(),
            headers: Default::default(),
        })),
    };

    println!("{}", serde_json::to_string_pretty(&kaka).unwrap());

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            tokio::spawn(async move {
               while let Some(msg) = rx.recv().await {
                   handle.emit("backend-message", msg).unwrap();
               }
            });
            app.manage(AppData {
                service: Box::new(api_service),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_endpoints, get_requests_by_endpoint_id, get_request, create_endpoint])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    webhook_server_join.join().expect("Thread panicked");
    api_server_join.join().expect("Thread panicked");
}
