mod model;

use crate::model::{
    EndpointCreateDto, EndpointResponseDto, ReadAllEndpointsResponseDto, ReadAllRequestsResponseDto,
};
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, error, get, post, web,
};
use derive_more::{Display, Error};
use domain::model::persistence::PersistenceError;
use domain::services::ApiServiceInterface;
use log::error;
use std::thread;
use tokio::sync::Mutex;

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display("Internal error")]
    UnwrapError(#[error(ignore)] String),
    #[display("Not found")]
    NotFound,
    AlreadyExists,
    InternalServerError,
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UnwrapError(e) => {
                error!("{e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AlreadyExists => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl From<PersistenceError> for ApiError {
    fn from(value: PersistenceError) -> Self {
        match value {
            PersistenceError::ResourceAlreadyExists => ApiError::AlreadyExists,
            PersistenceError::UnhandledError => ApiError::InternalServerError,
        }
    }
}

#[get("/api/v1/endpoints")]
async fn read_available_endpoints(
    _req: HttpRequest,
    data: Data<AppState>,
) -> Result<impl Responder> {
    let service = data.service.lock().await;
    Ok(web::Json(ReadAllEndpointsResponseDto::new(
        service.get_endpoints().await,
    )))
}

#[post("/api/v1/endpoints")]
async fn create_endpoint(
    endpoint_create_dto: web::Json<EndpointCreateDto>,
    data: Data<AppState>,
) -> Result<impl Responder> {
    let service = data.service.lock().await;
    let endpoint = service
        .create_endpoint(endpoint_create_dto.into_inner().url)
        .await
        .map(EndpointResponseDto::from)
        .map_err(ApiError::from)?;
    Ok(web::Json(endpoint))
}

#[get("/api/v1/{endpoint_url}/requests")]
async fn read_requests(data: Data<AppState>, name: web::Path<String>) -> Result<impl Responder> {
    //let endpoint_uuid = Uuid::parse_str(&name.into_inner()).map_err(|_| ApiError::NotFound)?;
    let service = data.service.lock().await;
    let endpoint_id = service
        .get_endpoint(name.into_inner())
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .ok_or(ApiError::NotFound)?
        .id;
    Ok(web::Json(ReadAllRequestsResponseDto::new(
        service.get_requests(endpoint_id).await,
    )))
}

#[post("/api/v1/endpoints/random")]
async fn create_random_endpoint(data: Data<AppState>) -> Result<impl Responder> {
    let service = data.service.lock().await;
    let endpoint = service
        .create_random_endpoint()
        .await
        .map(EndpointResponseDto::from)
        .map_err(ApiError::from)?;
    Ok(web::Json(endpoint))
}

struct AppState {
    pub service: Mutex<Box<dyn ApiServiceInterface>>,
}

pub fn start_server(url: &str, service: Box<dyn ApiServiceInterface>) -> thread::JoinHandle<()> {
    let addr = url.to_string();
    thread::spawn(move || {
        let sys = actix_web::rt::System::new();
        sys.block_on(async move {
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(AppState {
                        service: Mutex::new(service.clone()),
                    }))
                    .service(create_endpoint)
                    .service(read_requests)
                    .service(read_available_endpoints)
                    .service(create_random_endpoint)
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
