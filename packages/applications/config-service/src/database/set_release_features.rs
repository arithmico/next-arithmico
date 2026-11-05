use std::collections::{BTreeMap, BTreeSet};

use aws_sdk_dynamodb::types::AttributeValue;
use derive_builder::Builder;
use semver::Version;
use tracing::instrument;

use crate::database::DatabaseQuery;

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "crate::error::ServiceError"))]
pub struct SetReleaseFeatures {
    organization_id: u64,
    repository_id: u64,
    version: Version,
    features: BTreeMap<String, BTreeSet<String>>,
}

impl DatabaseQuery for SetReleaseFeatures {
    type Output = ();

    #[instrument(skip_all)]
    async fn execute(
        self,
        database: &super::Database,
    ) -> anyhow::Result<Self::Output> {
        let features = AttributeValue::M(
            self.features
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        AttributeValue::L(
                            value.into_iter().map(AttributeValue::S).collect(),
                        ),
                    )
                })
                .collect(),
        );

        database
            .client
            .update_item()
            .table_name(&database.table_name)
            .key(
                "PK",
                AttributeValue::S(format!("ORG#{}", self.organization_id)),
            )
            .key(
                "SK",
                AttributeValue::S(format!(
                    "REPO#{}#RELEASE#{}",
                    self.repository_id, self.version
                )),
            )
            .update_expression("SET #attr = :val")
            .expression_attribute_names("#attr", "features")
            .expression_attribute_values(":val", features)
            .send()
            .await?;

        Ok(())
    }
}
