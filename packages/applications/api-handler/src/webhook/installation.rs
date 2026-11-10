use anyhow::anyhow;
use database::{
    put_organization::PutOrganizationQueryBuilder,
    put_repository::PutRepositoryQueryBuilder,
};
use octocrab::models::webhook_events::{
    EventInstallation,
    payload::{
        InstallationWebhookEventAction, InstallationWebhookEventPayload,
    },
};
use tracing::{info, instrument, warn};

use crate::state::ServiceState;

#[instrument(skip_all)]
pub async fn handle_installation(
    state: &ServiceState,
    installation: &EventInstallation,
    payload: &InstallationWebhookEventPayload,
) -> anyhow::Result<()> {
    match &payload.action {
        InstallationWebhookEventAction::Created => {
            let EventInstallation::Full(installation) = installation else {
                return Err(anyhow!("Missing full installation information"));
            };

            if installation.account.r#type != "Organization" {
                return Err(anyhow!(
                    "Installation target is not an organization"
                ));
            }

            let organization_id = installation.account.id.0;
            let installation_id = installation.id.0;
            state
                .database
                .execute(
                    PutOrganizationQueryBuilder::default()
                        .installation_id(installation_id)
                        .organization_id(organization_id)
                        .build()?,
                )
                .await?;

            info!(
                message = "Installation created",
                organization_id, installation_id
            );

            for repository in payload.repositories.iter().flatten() {
                let repository_id = repository.id.0;
                state
                    .database
                    .execute(
                        PutRepositoryQueryBuilder::default()
                            .organization_id(organization_id)
                            .repository_id(repository_id)
                            .build()?,
                    )
                    .await?;

                info!(
                    message = "Repository created",
                    organization_id, repository_id
                );
            }
        }
        action => warn!(
            message = "Unsupported installation action",
            action = ?action
        ),
    }

    Ok(())
}
