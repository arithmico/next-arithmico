use std::sync::Arc;

use anyhow::anyhow;
use database::{
    Database, get_organization::GetOrganizationByOrganizationIdQueryBuilder,
    get_repository::GetRepositoryByRepositoryIdBuilder,
};
use github_app_auth::{GithubAppSecret, GithubInstallationTokenProvider};
use octocrab::Octocrab;
use tracing::{info, instrument};
use workflow_dispatcher_protocol::DispatchWorkflow;

use crate::config::Config;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Handler {
    github_installation_token_provider: GithubInstallationTokenProvider,
    database: Database,
    sqs_client: aws_sdk_sqs::Client,
    queue_url: String,
}

impl Handler {
    pub async fn init(config: Config) -> anyhow::Result<Self> {
        let sdk_config = aws_config::load_from_env().await;
        let github_app_secret = Arc::new(
            GithubAppSecret::load(&config.github_app_secret_arn, &sdk_config)
                .await?,
        );

        let github_installation_token_provider =
            GithubInstallationTokenProvider::connect(github_app_secret).await?;
        let database = Database::new(&sdk_config, &config.dynamodb_table_name);
        let sqs_client = aws_sdk_sqs::Client::new(&sdk_config);

        Ok(Self {
            github_installation_token_provider,
            database,
            sqs_client,
            queue_url: config.workflow_queue_url,
        })
    }

    #[instrument(skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        let response = self
            .sqs_client
            .receive_message()
            .queue_url(&self.queue_url)
            .max_number_of_messages(1)
            .send()
            .await?;

        let Some(messages) = response.messages else {
            return Ok(());
        };

        let Some(message) = messages.first() else {
            return Ok(());
        };

        let Some(receipt_handle) = message.receipt_handle() else {
            return Err(anyhow!("Message does not contain a receipt_handle"));
        };

        let body = message
            .body()
            .ok_or(anyhow!("Message does not have a body"))?;

        let message: DispatchWorkflow = serde_json::from_str(body)?;

        let repository = self
            .database
            .execute(
                GetRepositoryByRepositoryIdBuilder::default()
                    .repository_id(message.repository_id)
                    .build()?,
            )
            .await?;

        let organization = self
            .database
            .execute(
                GetOrganizationByOrganizationIdQueryBuilder::default()
                    .organization_id(repository.organization_id)
                    .build()?,
            )
            .await?;

        let access_token = self
            .github_installation_token_provider
            .get_installtion_access_token(organization.installation_id)
            .await?;

        let instance =
            Octocrab::builder().personal_token(access_token).build()?;

        let repo = instance.repos_by_id(repository.repository_id).get().await?;

        let owner = repo
            .owner
            .ok_or_else(|| anyhow!("Repository is missing the owner field"))?
            .login;
        let name = repo.name;

        instance
            .actions()
            .create_workflow_dispatch(
                owner,
                name,
                &message.workflow_id,
                &message.git_ref,
            )
            .send()
            .await?;

        self.sqs_client
            .delete_message()
            .queue_url(&self.queue_url)
            .receipt_handle(receipt_handle)
            .send()
            .await?;

        info!(
            message = "dispatched workflow run",
            repository_id = repository.repository_id,
            organization_id = organization.organization_id,
            git_ref = message.git_ref,
            workflow_id = message.workflow_id
        );

        Ok(())
    }
}
