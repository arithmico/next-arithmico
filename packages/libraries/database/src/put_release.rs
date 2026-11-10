use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;
use semver::Version;

use crate::DatabaseQuery;

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct PutReleaseQuery {
    organization_id: u64,
    repository_id: u64,
    version: Version,
}

impl DatabaseQuery for PutReleaseQuery {
    type Output = ();

    async fn execute(
        self,
        database: &super::Database,
    ) -> Result<Self::Output, crate::error::Error> {
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
                AttributeValue::S(format!(
                    "REPO#{}#RELEASE#{}",
                    self.repository_id, self.version
                )),
            )
            .item(
                "GSI1PK",
                AttributeValue::S(format!("REPO#{}", self.repository_id)),
            )
            .item(
                "GSI1SK",
                AttributeValue::S(format!("RELEASE#{}", self.version)),
            )
            .item(
                "organization_id",
                AttributeValue::S(self.organization_id.to_string()),
            )
            .item(
                "repository_id",
                AttributeValue::N(self.repository_id.to_string()),
            )
            .item("version", AttributeValue::S(self.version.to_string()))
            .item("type", AttributeValue::S(String::from("release")))
            .send()
            .await?;

        Ok(())
    }
}
