use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_dynamo::from_item;
use tracing::instrument;

use crate::DatabaseQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub installation_id: u64,
    pub organization_id: u64,
}

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct GetOrganizationByInstallationIdQuery {
    installation_id: u64,
}

impl DatabaseQuery for GetOrganizationByInstallationIdQuery {
    type Output = Organization;

    async fn execute(
        self,
        database: &super::Database,
    ) -> Result<Self::Output, crate::error::Error> {
        let response = database
            .client
            .query()
            .table_name(&database.table_name)
            .index_name("GSI1")
            .key_condition_expression("GSI1PK = :gsi1pk")
            .expression_attribute_values(
                ":gsi1pk",
                AttributeValue::S(format!(
                    "INSTALLATION#{}",
                    self.installation_id
                )),
            )
            .limit(1)
            .send()
            .await?;

        let item = response
            .items()
            .first()
            .ok_or(crate::error::Error::NotFound)?
            .clone();

        let organization: Organization = from_item(item)?;

        Ok(organization)
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct GetOrganizationByOrganizationIdQuery {
    organization_id: u64,
}

impl DatabaseQuery for GetOrganizationByOrganizationIdQuery {
    type Output = Organization;

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
            .key("SK", AttributeValue::S("METADATA".to_string()))
            .send()
            .await?;

        let Some(item) = response.item else {
            return Err(crate::error::Error::NotFound);
        };

        let organization: Organization = from_item(item)?;

        Ok(organization)
    }
}
