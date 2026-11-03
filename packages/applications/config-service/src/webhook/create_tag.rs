use anyhow::anyhow;
use octocrab::models::{
    Repository,
    webhook_events::{
        EventInstallation,
        payload::{CreateWebhookEventPayload, RefType},
    },
};
use semver::Version;
use tracing::{info, instrument};

use crate::{
    database::{
        get_organization::GetOrganizationByInstallationIdQueryBuilder,
        get_repository::GetRepositoryByOrganizationIdAndRepositoryIdBuilder,
        put_release::PutReleaseQueryBuilder,
    },
    state::ServiceState,
};

#[instrument(skip_all)]
pub async fn handle_create_tag(
    state: &ServiceState,
    installation: &EventInstallation,
    repository: &Repository,
    payload: &CreateWebhookEventPayload,
) -> anyhow::Result<()> {
    if payload.ref_type != RefType::Tag {
        return Err(anyhow!("Unsupported ref type: {:?}", payload.ref_type));
    }

    let installation_id = installation.id().0;
    let organization = state
        .database
        .execute(
            GetOrganizationByInstallationIdQueryBuilder::default()
                .installation_id(installation_id)
                .build()?,
        )
        .await?;

    let repository_id = repository.id.0;
    let repository = state
        .database
        .execute(
            GetRepositoryByOrganizationIdAndRepositoryIdBuilder::default()
                .organization_id(organization.organization_id)
                .repository_id(repository_id)
                .build()?,
        )
        .await?;

    let version = payload.r#ref.strip_prefix("v").unwrap_or(&payload.r#ref);
    let version = Version::parse(version)?;

    state
        .database
        .execute(
            PutReleaseQueryBuilder::default()
                .organization_id(organization.organization_id)
                .repository_id(repository.repository_id)
                .version(version.clone())
                .build()?,
        )
        .await?;

    info!(
        message = "Created release",
        organization_id = organization.organization_id,
        repository_id,
        version = ?version
    );

    Ok(())
}
