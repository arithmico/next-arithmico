use crate::{
    config::Config, database::Database, state::github_oidc::GithubOidcClient,
};
use anyhow::anyhow;
use tracing::instrument;

mod github_oidc;

#[derive(Debug, Clone)]
pub struct ServiceState {
    pub(crate) webhook_secret: String,
    #[allow(dead_code)]
    pub(crate) github_app_id: String,
    pub(crate) database: Database,
    pub(crate) github_oidc_client: GithubOidcClient,
}

impl ServiceState {
    #[instrument]
    pub async fn load(config: &Config) -> anyhow::Result<Self> {
        let sdk_config = aws_config::load_from_env().await;
        let sm_client = aws_sdk_secretsmanager::Client::new(&sdk_config);
        let webhook_secret = sm_client
            .get_secret_value()
            .secret_id(&config.webhook_secret_arn)
            .send()
            .await?
            .secret_string
            .ok_or_else(|| anyhow!("Webhook secret is empty"))?;

        let github_app_id = sm_client
            .get_secret_value()
            .secret_id(&config.github_app_id_secret_arn)
            .send()
            .await?
            .secret_string
            .ok_or_else(|| anyhow!("Github app id secret is empty"))?;

        let database = Database::new(&sdk_config, &config.dynamodb_table_name);

        let github_oidc_client =
            GithubOidcClient::init(&config.github_oidc_configuration_uri)
                .await?;

        Ok(Self {
            webhook_secret,
            database,
            github_oidc_client,
            github_app_id,
        })
    }
}
