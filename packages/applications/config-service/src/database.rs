use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Database {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

impl Database {
    pub fn new(sdk_config: &SdkConfig, table_name: &str) -> Self {
        let client = aws_sdk_dynamodb::Client::new(sdk_config);
        Self {
            client,
            table_name: table_name.to_string(),
        }
    }

    #[instrument]
    pub async fn put_installation(
        &self,
        installation_id: i64,
        account: &str,
    ) -> anyhow::Result<()> {
        self.client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", AttributeValue::S(format!("ACCOUNT#{}", account)))
            .item(
                "SK",
                AttributeValue::S(format!("INSTALLATION#{}", installation_id)),
            )
            .item(
                "installation_id",
                AttributeValue::S(installation_id.to_string()),
            )
            .item("account", AttributeValue::S(account.to_string()))
            .send()
            .await?;

        Ok(())
    }
}
