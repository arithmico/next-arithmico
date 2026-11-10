use aws_config::SdkConfig;
use tracing::instrument;

pub mod get_organization;
pub mod get_repository;
pub mod put_organization;
pub mod put_release;
pub mod put_repository;
pub mod set_release_features;

#[derive(Debug, Clone)]
pub struct Database {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

#[allow(async_fn_in_trait)]
pub trait DatabaseQuery {
    type Output;

    async fn execute(self, database: &Database)
    -> anyhow::Result<Self::Output>;
}

impl Database {
    pub fn new(sdk_config: &SdkConfig, table_name: &str) -> Self {
        let client = aws_sdk_dynamodb::Client::new(sdk_config);
        Self {
            client,
            table_name: table_name.to_string(),
        }
    }

    #[instrument(skip_all)]
    pub async fn execute<Q: DatabaseQuery>(
        &self,
        query: Q,
    ) -> anyhow::Result<Q::Output> {
        query.execute(self).await
    }
}
