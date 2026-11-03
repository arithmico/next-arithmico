use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;

use crate::database::DatabaseQuery;

#[derive(Debug, Clone, Builder)]
pub struct PutOrganizationQuery {
    organization_id: u64,
    installation_id: u64,
}

impl DatabaseQuery for PutOrganizationQuery {
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
            .item("SK", AttributeValue::S("METADATA".to_string()))
            .item(
                "GSI1PK",
                AttributeValue::S(format!(
                    "INSTALLATION#{}",
                    self.installation_id
                )),
            )
            .item(
                "GSI1SK",
                AttributeValue::S(format!("ORG#{}", self.organization_id)),
            )
            .item(
                "installation_id",
                AttributeValue::S(self.installation_id.to_string()),
            )
            .item(
                "organization_id",
                AttributeValue::S(self.organization_id.to_string()),
            )
            .item("type", AttributeValue::S(String::from("organization")))
            .send()
            .await?;
        Ok(())
    }
}
