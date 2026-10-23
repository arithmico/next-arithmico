use std::path::{Path, PathBuf};

use anyhow::anyhow;
use cargo_metadata::MetadataCommand;

use crate::feature::{Feature, read_crate_features};

pub fn verify() -> anyhow::Result<()> {
    let metadata = MetadataCommand::new().exec()?;
    let workspace_root = metadata.workspace_root.into_std_path_buf();
    let evaluator_features = read_evaluator_features(&workspace_root)?;
    let engine_features = read_engine_features(&workspace_root)?;
    for feature in evaluator_features.iter() {
        if let Some(engine_feature) =
            engine_features.iter().find(|f| f.name == feature.name)
        {
            if !engine_feature
                .dependencies
                .contains(&format!("evaluator/{}", feature.name))
            {
                return Err(anyhow!(
                    r#"Engine feature "{0}" is missing the depdendency "evaluator/{0}"!"#,
                    feature.name
                ));
            }
        } else {
            return Err(anyhow!(
                r#"Evaluator feature "{}" is not forwarded by engine!"#,
                feature.name
            ));
        }
    }
    Ok(())
}

fn read_engine_features(workspace_root: &Path) -> anyhow::Result<Vec<Feature>> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/engine/Cargo.toml");
    read_crate_features(&path)
}

fn read_evaluator_features(
    workspace_root: &Path,
) -> anyhow::Result<Vec<Feature>> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/evaluator/Cargo.toml");
    read_crate_features(&path)
}
