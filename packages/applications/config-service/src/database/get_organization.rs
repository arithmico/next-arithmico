use anyhow::anyhow;
use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_dynamo::from_item;

use crate::database::DatabaseQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub installation_id: u64,
    pub organization_id: u64,
}

#[derive(Debug, Clone, Builder)]
pub struct GetOrganizationByInstallationIdQuery {
    installation_id: u64,
}

impl DatabaseQuery for GetOrganizationByInstallationIdQuery {
    type Output = Organization;

    async fn execute(
        self,
        database: &super::Database,
    ) -> anyhow::Result<Self::Output> {
        let response = database
            .client
            .query()
            .table_name(&database.table_name)
            .index_name("GSI1")
            .key_condition_expression("GSI1PK = :gsi1pk")
            .expression_attribute_values(
                ":gsi1pk",
                AttributeValue::S(self.installation_id.to_string()),
            )
            .limit(1)
            .send()
            .await?;

        let item = response
            .items()
            .first()
            .ok_or_else(|| anyhow!("organization not found"))?
            .clone();

        let organization: Organization = from_item(item)?;

        Ok(organization)
    }
}
