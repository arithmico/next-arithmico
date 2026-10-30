use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use tracing::{error, info, instrument, warn};

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

    if let Some(action) = payload.get("action").and_then(|v| v.as_str()) {
        if action == "created" {
            let Some(installation_id) = payload["installation"]["id"]
                .as_number()
                .and_then(|n| n.as_i64())
            else {
                error!("Invalid or missing installation id");
                return StatusCode::BAD_REQUEST;
            };
            let Some(account) = payload["account"]["login"].as_str() else {
                error!("Invalid or missing account.login field");
                return StatusCode::BAD_REQUEST;
            };
            if let Err(err) = state
                .database
                .put_installation(installation_id, account)
                .await
            {
                error!("Installation failed: {}", err);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        } else {
            warn!("Unknown action type: {}", action);
        }
    } else {
        info!(message = "Unknown webhook type", body = ?payload);
    }

    StatusCode::OK
}
