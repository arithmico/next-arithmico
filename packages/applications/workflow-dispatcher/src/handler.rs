use std::sync::Arc;

use database::Database;
use github_app_auth::{GithubAppSecret, GithubInstallationTokenProvider};

use crate::config::Config;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Handler {
    github_installation_token_provider: GithubInstallationTokenProvider,
    database: Database,
    sqs_client: aws_sdk_sqs::Client,
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
        })
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
