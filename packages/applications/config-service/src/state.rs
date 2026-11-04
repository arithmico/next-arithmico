use crate::{
    config::Config, database::Database, state::github_oidc::GithubOidcClient,
};
use anyhow::anyhow;
use tracing::instrument;

mod github_oidc;

#[derive(Debug, Clone)]
pub struct ServiceState {
    pub(crate) github_webhook_secret: String,
    #[allow(dead_code)]
    pub(crate) github_client_id: String,
    pub(crate) database: Database,
    pub(crate) github_oidc_client: GithubOidcClient,
}

impl ServiceState {
    #[instrument]
    pub async fn load(config: &Config) -> anyhow::Result<Self> {
        let sdk_config = aws_config::load_from_env().await;
        let sm_client = aws_sdk_secretsmanager::Client::new(&sdk_config);

        let github_app_secret = sm_client
            .get_secret_value()
            .secret_id(&config.github_app_secret_arn)
            .send()
            .await?
            .secret_string
            .ok_or_else(|| anyhow!("Github client id secret is empty"))?;

        let github_app_secret: serde_json::Value =
            serde_json::from_str(&github_app_secret)?;

        let github_webhook_secret = github_app_secret
            .get("webhook_secret")
            .ok_or_else(|| {
                anyhow!("Github app secret is missing the webhook_secret field")
            })?
            .as_str()
            .ok_or_else(|| anyhow!("webhook_secret is not a string"))?
            .to_string();

        let github_client_id = github_app_secret
            .get("webhook_secret")
            .ok_or_else(|| {
                anyhow!("Github app secret is missing the client_id field")
            })?
            .as_str()
            .ok_or_else(|| anyhow!("client_id is not a string"))?
            .to_string();

        let database = Database::new(&sdk_config, &config.dynamodb_table_name);

        let github_oidc_client =
            GithubOidcClient::init(&config.github_oidc_configuration_uri)
                .await?;

        Ok(Self {
            github_webhook_secret,
            database,
            github_oidc_client,
            github_client_id,
        })
    }
}
