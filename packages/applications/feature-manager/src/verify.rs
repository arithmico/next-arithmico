use std::{fs::File, io::Read};

pub fn verify() -> anyhow::Result<()> {
    let mut workspace_file = File::open("./Cargo.toml")?;
    let mut content = String::new();
    workspace_file.read_to_string(&mut content)?;

    Ok(())
}
