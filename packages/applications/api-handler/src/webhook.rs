use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload};
use tracing::{error, instrument, warn};

use crate::{
    state::ServiceState,
    webhook::{
        create_tag::handle_create_tag, installation::handle_installation,
    },
};

mod create_tag;
mod installation;

#[instrument(skip_all)]
pub async fn handler(
    State(state): State<ServiceState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let Some(event) = headers.get("X-GitHub-Event") else {
        error!("Missing X-GitHub-Event header");
        return StatusCode::BAD_REQUEST;
    };
    let Ok(event) = event.to_str() else {
        error!("X-GitHub-Event header value is not a valid string");
        return StatusCode::BAD_REQUEST;
    };
    let Some(signature) = headers.get("X-Hub-Signature-256") else {
        error!("Missing X-Hub-Signature-256 header");
        return StatusCode::BAD_REQUEST;
    };
    let Ok(signature) = signature.to_str() else {
        error!("Signature is not a valid string");
        return StatusCode::BAD_REQUEST;
    };

    let event = match WebhookEvent::try_from_header_signature_and_body(
        event,
        signature,
        &state.github_webhook_secret,
        &body,
    ) {
        Ok(event) => event,
        Err(err) => {
            error!(message = "Failed to deserialize event", error = ?err);
            return StatusCode::BAD_REQUEST;
        }
    };

    let Some(installation) = event.installation else {
        error!("Missing installation information");
        return StatusCode::BAD_REQUEST;
    };

    match event.specific {
        WebhookEventPayload::Installation(payload) => {
            if let Err(err) =
                handle_installation(&state, &installation, &payload).await
            {
                error!(
                    message = "Failed to process installation event",
                    error = ?err
                );
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        WebhookEventPayload::Create(payload) => {
            let Some(repository) = event.repository else {
                error!(message = "Missing repository information");
                return StatusCode::BAD_REQUEST;
            };

            if let Err(err) =
                handle_create_tag(&state, &installation, &repository, &payload)
                    .await
            {
                error!(
                    message = "Failed to process create event",
                    error = ?err
                );
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        _ => {
            warn!("Unsupported webhook type: {:?}", event.kind)
        }
    }

    StatusCode::OK
}
