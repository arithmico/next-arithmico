use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;

use crate::database::DatabaseQuery;

#[derive(Debug, Clone, Builder)]
pub struct PutRepositoryQuery {
    repository_id: u64,
    organization_id: u64,
}

impl DatabaseQuery for PutRepositoryQuery {
    type Output = ();

    async fn execute(
        self,
        database: &super::Database,
    ) -> anyhow::Result<Self::Output> {
        database
            .client
            .put_item()
            .table_name(&database.table_name)
            .item(
                "PK",
                AttributeValue::S(format!("ORG#{}", self.organization_id)),
            )
            .item(
                "SK",
                AttributeValue::S(format!("REPO#{}", self.repository_id)),
            )
            .item(
                "GSI1PK",
                AttributeValue::S(format!("REPO#{}", self.repository_id)),
            )
            .item(
                "GSI1SK",
                AttributeValue::S(format!("ORG#{}", self.organization_id)),
            )
            .item(
                "repository_id",
                AttributeValue::N(self.repository_id.to_string()),
            )
            .item(
                "organization_id",
                AttributeValue::N(self.organization_id.to_string()),
            )
            .item("type", AttributeValue::S(String::from("repository")))
            .send()
            .await?;
        Ok(())
    }
}
