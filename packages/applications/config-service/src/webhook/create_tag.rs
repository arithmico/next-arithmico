use octocrab::models::webhook_events::{
    EventInstallation, payload::CreateWebhookEventPayload,
};
use tracing::instrument;

use crate::state::ServiceState;

#[instrument(skip_all)]
pub async fn handle_create_tag(
    _state: &ServiceState,
    _installation: &EventInstallation,
    _payload: &CreateWebhookEventPayload,
) -> anyhow::Result<()> {
    Ok(())
}
