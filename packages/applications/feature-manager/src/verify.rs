use std::path::{Path, PathBuf};

use cargo_metadata::MetadataCommand;

use crate::feature::CrateFeatures;

pub fn verify() -> anyhow::Result<()> {
    let metadata = MetadataCommand::new().exec()?;
    let workspace_root = metadata.workspace_root.into_std_path_buf();
    let evaluator = read_evaluator_features(&workspace_root)?;
    let engine = read_engine_features(&workspace_root)?;
    let engine_worker = read_worker_features(&workspace_root)?;

    evaluator.validate_no_features_disabled_by_default()?;
    engine.validate_no_features_disabled_by_default()?;
    engine_worker.validate_no_features_disabled_by_default()?;

    engine_worker.validate_forwards_features_of(&evaluator)?;
    engine_worker.validate_forwards_features_of(&engine)?;

    Ok(())
}

fn read_worker_features(
    workspace_root: &Path,
) -> anyhow::Result<CrateFeatures> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/applications/engine-worker/Cargo.toml");
    CrateFeatures::read(&path)
}

fn read_engine_features(
    workspace_root: &Path,
) -> anyhow::Result<CrateFeatures> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/engine/Cargo.toml");
    CrateFeatures::read(&path)
}

fn read_evaluator_features(
    workspace_root: &Path,
) -> anyhow::Result<CrateFeatures> {
    let mut path = PathBuf::from(workspace_root);
    path.push("packages/libraries/evaluator/Cargo.toml");
    CrateFeatures::read(&path)
}
