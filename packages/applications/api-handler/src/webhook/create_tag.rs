use anyhow::anyhow;
use database::{
    get_organization::GetOrganizationByInstallationIdQueryBuilder,
    get_repository::GetRepositoryByOrganizationIdAndRepositoryIdBuilder,
    put_release::PutReleaseQueryBuilder,
};
use octocrab::{
    Octocrab,
    models::{
        Repository,
        webhook_events::{
            EventInstallation,
            payload::{CreateWebhookEventPayload, RefType},
        },
    },
};
use semver::Version;
use tracing::{info, instrument};
use workflow_dispatcher_protocol::DispatchWorkflow;

use crate::state::ServiceState;

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

    let tag = payload.r#ref.clone();
    let version = tag.strip_prefix("v").unwrap_or(&tag);
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

    state
        .workflow_publisher
        .dispatch_workflow(DispatchWorkflow {
            repository_id,
            workflow_id: "report-tag-metadata.yml".to_string(),
            git_ref: tag.clone(),
        })
        .await?;

    let access_token = state
        .github_installation_token_provider
        .get_installtion_access_token(installation_id)
        .await?;

    let instance = Octocrab::builder().personal_token(access_token).build()?;

    let repo = instance.repos_by_id(repository_id).get().await?;

    let owner = repo
        .owner
        .ok_or_else(|| anyhow!("Repository is missing the owner field"))?
        .login;
    let name = repo.name;

    instance
        .actions()
        .create_workflow_dispatch(owner, name, "report-tag-metadata.yml", tag)
        .send()
        .await?;

    Ok(())
}
