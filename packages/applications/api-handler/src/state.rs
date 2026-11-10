use std::sync::Arc;

use crate::{
    config::Config,
    state::{
        github_oidc::GithubOidcClient, workflow_publisher::WorkflowPublisher,
    },
};
use database::Database;
use github_app_auth::{GithubAppSecret, GithubInstallationTokenProvider};
use tracing::instrument;

mod github_oidc;
mod workflow_publisher;

#[derive(Debug, Clone)]
pub struct ServiceState {
    pub github_app_secret: Arc<GithubAppSecret>,
    pub github_installation_token_provider: GithubInstallationTokenProvider,
    pub database: Database,
    pub github_oidc_client: GithubOidcClient,
    pub workflow_publisher: WorkflowPublisher,
}

impl ServiceState {
    #[instrument]
    pub async fn load(config: &Config) -> anyhow::Result<Self> {
        let sdk_config = aws_config::load_from_env().await;

        let github_app_secret = Arc::new(
            GithubAppSecret::load(&config.github_app_secret_arn, &sdk_config)
                .await?,
        );

        let database = Database::new(&sdk_config, &config.dynamodb_table_name);

        let github_oidc_client =
            GithubOidcClient::init(&config.github_oidc_configuration_uri)
                .await?;

        let github_installation_token_provider =
            GithubInstallationTokenProvider::connect(github_app_secret.clone())
                .await?;

        let workflow_publisher =
            WorkflowPublisher::new(&config.workflow_queue_url, &sdk_config);

        Ok(Self {
            github_app_secret,
            database,
            github_oidc_client,
            github_installation_token_provider,
            workflow_publisher,
        })
    }
}
