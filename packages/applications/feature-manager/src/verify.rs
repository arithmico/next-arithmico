use std::{fs::File, io::Read};

use cargo_metadata::MetadataCommand;

pub fn verify() -> anyhow::Result<()> {
    let metadata = MetadataCommand::new().exec()?;
    let mut workspace_file_path = metadata.workspace_root.into_std_path_buf();
    workspace_file_path.push("Cargo.toml");
    let mut workspace_file = File::open(&workspace_file_path)?;
    let mut content = String::new();
    workspace_file.read_to_string(&mut content)?;

    Ok(())
}
