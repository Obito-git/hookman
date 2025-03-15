use crate::option_or_webhook_err;
use crate::{AppState, WebhookError};
use actix_web::web::Data;
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, connect, delete, get, head, options, patch,
    post, put, trace,
};
use chrono::Utc;
use domain::model::webhook::{HttpMethod, WebhookRequest};
use itertools::Itertools;
use serde_json::json;
use tracing::debug;
use domain::model::endpoint::EndpointReadDto;

/// can have duplicate header name, saves all
/// peer addr can be None only in unit tests
/// TODO: handle method here, but not via param?
#[tracing::instrument]
pub fn http_to_webhook_request(
    req: HttpRequest,
    body: String,
    http_method: HttpMethod,
) -> Result<WebhookRequest, WebhookError> {
    debug!("New request received");
    let host = option_or_webhook_err!(req.peer_addr());

    let params: Vec<serde_json::Value> = req
        .query_string()
        .split('&')
        .filter_map(|val| {
            if let Some((k, v)) = val.split_once('=') {
                let (k, v) = (k.trim(), v.trim());
                if k.is_empty()
                    || k.chars().all(char::is_whitespace)
                    || v.is_empty()
                    || v.chars().all(char::is_whitespace)
                {
                    return None;
                }
                Some(json!({"key": k, "value": v}))
            } else {
                None
            }
        })
        .collect();
    let json_headers: Vec<serde_json::Value> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(k, v)| json!({ "key": k, "value": v }))
        .collect();

    Ok(WebhookRequest {
        host: host.ip().to_string(),
        headers: serde_json::Value::Array(json_headers),
        body: (!body.is_empty()).then_some(body),
        query_params: (!params.is_empty()).then_some(serde_json::Value::Array(params)),
        timestamp: Utc::now(),
        http_method,
    })
}

async fn handle_webhook(
    http_request: HttpRequest,
    data: Data<AppState>,
    body: String,
    http_method: HttpMethod,
) -> Result<impl Responder, WebhookError> {
    let endpoint = option_or_webhook_err!(http_request.extensions_mut().remove::<EndpointReadDto>());
    let mut lock = data.service.lock().await;
    lock.process_request(
        endpoint,
        http_to_webhook_request(http_request, body, http_method)?,
    )
    .await;
    Ok(HttpResponse::Ok())
}

#[get("/{path}")]
async fn handle_get(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Get).await
}

#[post("/{path}")]
async fn handle_post(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Post).await
}

#[delete("/{path}")]
async fn handle_delete(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Delete).await
}

#[put("/{path}")]
async fn handle_put(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Put).await
}

#[patch("/{path}")]
async fn handle_patch(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Patch).await
}

#[head("/{path}")]
async fn handle_head(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Head).await
}

#[connect("/{path}")]
async fn handle_connect(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Connect).await
}

#[options("/{path}")]
async fn handle_options(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Options).await
}

#[trace("/{path}")]
async fn handle_trace(
    req: HttpRequest,
    data: Data<AppState>,
    body: String,
) -> Result<impl Responder, WebhookError> {
    handle_webhook(req, data, body, HttpMethod::Trace).await
}
