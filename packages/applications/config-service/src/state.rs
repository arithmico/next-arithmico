use crate::{config::Config, database::Database};
use anyhow::anyhow;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct ServiceState {
    pub webhook_secret: String,
    pub database: Database,
}

impl ServiceState {
    #[instrument]
    pub async fn load(config: &Config) -> anyhow::Result<Self> {
        let sdk_config = aws_config::load_from_env().await;
        let sm_client = aws_sdk_secretsmanager::Client::new(&sdk_config);
        let response = sm_client
            .get_secret_value()
            .secret_id(&config.webhook_secret_arn)
            .send()
            .await?;

        let webhook_secret = response
            .secret_string()
            .ok_or_else(|| anyhow!("Webhook secret is empty"))?
            .to_string();

        let database = Database::new(&sdk_config, &config.dynamodb_table_name);

        Ok(Self {
            webhook_secret,
            database,
        })
    }
}
