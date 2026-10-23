use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use cargo_metadata::MetadataCommand;
use toml_edit::Document;

pub fn verify() -> anyhow::Result<()> {
    let metadata = MetadataCommand::new().exec()?;
    let workspace_root = metadata.workspace_root.into_std_path_buf();
    let evaluator_features = read_evaluator_features(&workspace_root)?;
    let engine_features = read_engine_features(&workspace_root)?;
    for feature in evaluator_features.iter() {
        if !engine_features.contains(feature) {
            return Err(anyhow!(
                r#"Evaluator feature "{}" is not forwarded by engine!"#,
                feature
            ));
        }
    }
    Ok(())
}

fn read_engine_features(workspace_root: &Path) -> anyhow::Result<Vec<String>> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/engine/Cargo.toml");
    read_crate_features(&path)
}

fn read_evaluator_features(
    workspace_root: &Path,
) -> anyhow::Result<Vec<String>> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/evaluator/Cargo.toml");
    read_crate_features(&path)
}

fn read_crate_features(path: &Path) -> anyhow::Result<Vec<String>> {
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
        .map(|(name, _)| name.to_string())
        .collect())
}
