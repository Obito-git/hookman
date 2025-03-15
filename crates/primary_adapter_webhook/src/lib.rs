use crate::error_response::WebhookError;
use crate::routes::{
    handle_connect, handle_delete, handle_get, handle_head, handle_options, handle_patch,
    handle_post, handle_put, handle_trace,
};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{Next, from_fn};
use actix_web::web::Data;
use actix_web::{App, Error, HttpMessage, HttpServer};
use domain::services::WebhookServiceInterface;
use std::thread;
use tokio::sync::Mutex;
use uuid::Uuid;

mod error_response;
mod macros;
mod routes;

struct AppState {
    pub service: Mutex<Box<dyn WebhookServiceInterface>>,
}

async fn check_endpoint_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let app_data = option_or_webhook_err!(req.app_data::<Data<AppState>>());
    let endpoint = {
        let lock = app_data.service.lock().await;
        let path =
            Uuid::parse_str(req.path().trim_matches('/')).map_err(|_| WebhookError::NotFound)?;
        lock.get_endpoint(path)
            .await
            .map_err(|_| WebhookError::NotFound)?
    };
    match endpoint {
        Some(endpoint) => {
            req.extensions_mut().insert(endpoint);
            let res = next.call(req).await?;
            Ok(res)
        }
        None => Err(WebhookError::NotFound.into()),
    }
}

pub fn start_server(
    url: &str,
    service: Box<dyn WebhookServiceInterface>,
) -> thread::JoinHandle<()> {
    let addr = url.to_string();
    thread::spawn(move || {
        let sys = actix_web::rt::System::new();
        sys.block_on(async move {
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(AppState {
                        service: Mutex::new(service.clone()),
                    }))
                    .service(handle_get)
                    .service(handle_post)
                    .service(handle_put)
                    .service(handle_delete)
                    .service(handle_patch)
                    .service(handle_head)
                    .service(handle_connect)
                    .service(handle_options)
                    .service(handle_trace)
                    .wrap(from_fn(check_endpoint_middleware))
            })
            .bind(&addr)
            .expect("Failed to bind address")
            .workers(4)
            .run()
            .await
            .expect("Server error");
        });
    })
}
