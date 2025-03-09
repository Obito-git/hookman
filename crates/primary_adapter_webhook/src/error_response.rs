use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, error};
use derive_more::{Display, Error};
use log::error;

#[derive(Debug, Display, Error)]
pub enum WebhookError {
    #[display("Internal error")]
    UnwrapError(#[error(ignore)] String),
    #[display("Not found")]
    NotFound,
}

impl error::ResponseError for WebhookError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebhookError::UnwrapError(e) => {
                error!("{e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            WebhookError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }
}
