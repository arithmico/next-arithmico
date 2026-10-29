use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use tracing::{error, info, instrument};

use crate::state::ServiceState;

#[instrument]
pub async fn handler(
    State(state): State<ServiceState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let Some(signature) = headers.get("X-Hub-Signature-256") else {
        error!("Missing X-Hub-Signature-256 header");
        return StatusCode::BAD_REQUEST;
    };
    let Ok(signature) = signature.to_str() else {
        error!("Signature is not a valid string");
        return StatusCode::BAD_REQUEST;
    };
    let Some(signature) = signature.strip_prefix("sha256=") else {
        error!("Header value does not contain a signature");
        return StatusCode::BAD_REQUEST;
    };
    let Ok(signature) = hex::decode(signature) else {
        error!("Failed to decode hex signature");
        return StatusCode::BAD_REQUEST;
    };
    let Ok(mut mac) =
        Hmac::<Sha256>::new_from_slice(state.webhook_secret.as_bytes())
    else {
        error!("Failed to initialize HMAC");
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    mac.update(&body);
    if mac.verify_slice(&signature).is_err() {
        error!("Invalid signature");
        return StatusCode::UNAUTHORIZED;
    }

    let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&body) else {
        error!("Failed to parse json");
        return StatusCode::BAD_REQUEST;
    };

    info!(body = ?payload);

    StatusCode::OK
}
