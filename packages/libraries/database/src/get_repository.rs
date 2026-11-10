use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use tracing::instrument;

use crate::DatabaseQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "PK")]
    pub pk: String,

    #[serde(rename = "SK")]
    pub sk: String,

    #[serde(rename = "GSI1PK")]
    pub gsi1pk: String,

    #[serde(rename = "GSI1SK")]
    pub gsi1sk: String,

    pub repository_id: u64,

    pub organization_id: u64,
}

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct GetRepositoryByOrganizationIdAndRepositoryId {
    organization_id: u64,
    repository_id: u64,
}

impl DatabaseQuery for GetRepositoryByOrganizationIdAndRepositoryId {
    type Output = Repository;

    #[instrument(skip_all)]
    async fn execute(
        self,
        database: &super::Database,
    ) -> Result<Self::Output, crate::error::Error> {
        let response = database
            .client
            .get_item()
            .table_name(&database.table_name)
            .key(
                "PK",
                AttributeValue::S(format!("ORG#{}", self.organization_id)),
            )
            .key(
                "SK",
                AttributeValue::S(format!("REPO#{}", self.repository_id)),
            )
            .send()
            .await?;

        let Some(item) = response.item else {
            return Err(crate::error::Error::NotFound);
        };

        let repository: Repository = from_item(item)?;

        Ok(repository)
    }
}
