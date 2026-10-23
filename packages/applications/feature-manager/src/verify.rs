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
    let features = read_evaluator_features(&workspace_root)?;
    println!("Features = {:#?}", features);
    Ok(())
}

fn read_evaluator_features(
    workspace_root: &Path,
) -> anyhow::Result<Vec<String>> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/evaluator/Cargo.toml");
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;
    let document = content.parse::<Document<_>>()?;
    let features = document
        .get("features")
        .ok_or(anyhow!("Failed to read features"))?
        .as_table()
        .ok_or(anyhow!("Failed to read features array"))?;

    Ok(features.iter().map(|(name, _)| name.to_string()).collect())
}
