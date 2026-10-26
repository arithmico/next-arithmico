use std::{collections::BTreeMap, fs::File, io::Read, path::Path};

use anyhow::anyhow;
use toml_edit::{Document, Value};

#[derive(Debug)]
pub struct CrateFeatures {
    pub name: String,
    pub default: Vec<String>,
    pub features: BTreeMap<String, Vec<String>>,
}

impl CrateFeatures {
    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let mut content = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;
        let document = content.parse::<Document<_>>()?;
        let features = document
            .get("features")
            .ok_or(anyhow!("Failed to read features"))?
            .as_table()
            .ok_or(anyhow!("Failed to read features array"))?;

        let name = document
            .get("package")
            .and_then(|package| package.get("name"))
            .ok_or(anyhow!("Failed to read crate name"))?
            .as_str()
            .ok_or(anyhow!("Failed to parse crate name"))?
            .to_string();

        let mut output = Self {
            name,
            default: Vec::new(),
            features: BTreeMap::new(),
        };
        for (feature, dependencies) in features.into_iter() {
            let deps = if feature == "default" {
                &mut output.default
            } else {
                output.features.entry(feature.to_string()).or_default()
            };
            if let Some(dependencies) = dependencies.as_array() {
                for dependency in dependencies {
                    match dependency {
                        Value::String(dependency) => {
                            deps.push(dependency.value().clone());
                        }
                        dep => {
                            return Err(anyhow!(
                                r#"Feature "{}" has an invalid dependency: {}"#,
                                feature,
                                dep
                            ));
                        }
                    }
                }
            } else {
                return Err(anyhow!(
                    r#"Feature "{}" is missing a dependency array."#,
                    feature
                ));
            }
        }

        Ok(output)
    }

    pub fn validate_no_features_disabled_by_default(
        &self,
    ) -> anyhow::Result<()> {
        if let Some((feature, _)) = self
            .features
            .iter()
            .find(|(feature, _)| !self.default.contains(feature))
        {
            Err(anyhow!(
                r#"Feature "{}" is not enabled by default in crate "{}"."#,
                feature,
                self.name
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_forwards_features_of(
        &self,
        other: &Self,
    ) -> anyhow::Result<()> {
        for feature in other.features.keys() {
            let Some(deps) = self.features.get(feature) else {
                return Err(anyhow!(
                    r#"Crate "{}" is not forwarding feature "{}" of crate "{}""#,
                    self.name,
                    feature,
                    other.name
                ));
            };

            if !deps.contains(&format!("{}/{}", other.name, feature)) {
                return Err(anyhow!(
                    r#"Feature "{0}" in create "{1}" is missing the dependency "{2}/{0}""#,
                    feature,
                    self.name,
                    other.name
                ));
            }
        }
        Ok(())
    }
}
