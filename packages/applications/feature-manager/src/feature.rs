use std::{fs::File, io::Read, path::Path};

use anyhow::anyhow;
use toml_edit::{Document, Value};

#[derive(Debug)]
pub struct Feature {
    pub name: String,
    pub dependencies: Vec<String>,
}

impl Feature {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            dependencies: Vec::new(),
        }
    }
}

pub fn read_crate_features(path: &Path) -> anyhow::Result<Vec<Feature>> {
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;
    let document = content.parse::<Document<_>>()?;
    let features = document
        .get("features")
        .ok_or(anyhow!("Failed to read features"))?
        .as_table()
        .ok_or(anyhow!("Failed to read features array"))?;

    Ok(features
        .iter()
        .filter(|(name, _)| *name != "default")
        .map(|(name, dependencies)| {
            let mut feature = Feature::new(name);
            if let Some(dependencies) = dependencies.as_array() {
                for dependency in dependencies {
                    if let Value::String(dependency) = dependency {
                        feature.dependencies.push(dependency.value().clone());
                    }
                }
            }
            feature
        })
        .collect())
}
