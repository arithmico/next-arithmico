use std::clone;

use git2::{Oid, Repository};
use semver::Version;

fn main() {
    match run() {
        Ok(_) => (),
        Err(error) => println!("Error: {}", error.to_string()),
    }
}

fn run() -> anyhow::Result<()> {
    let repository = Repository::open_from_env()?;
    let last_version = find_last_version_tag(&repository);
    println!("tags = {:#?}", last_version);
    Ok(())
}

#[derive(Debug, Clone)]
struct VersionTag {
    version: Version,
    name: String,
    oid: Oid,
}

fn find_last_version_tag(
    repository: &Repository,
) -> anyhow::Result<VersionTag> {
    let mut versions = Vec::<VersionTag>::new();

    repository
        .tag_foreach(|oid, name| {
            let name = String::from_utf8(name.to_vec()).expect("name");
            println!("{name}");
            if let Some(name) = name.strip_prefix("refs/tags/v") {
                if let Ok(version) = Version::parse(&name) {
                    versions.push(VersionTag {
                        name: name.to_string(),
                        oid,
                        version,
                    });
                }
            }
            true
        })
        .expect("all tags checked");

    versions.sort_by(|a, b| a.version.cmp(&b.version));
    let last = versions.last().cloned().expect("last");

    Ok(last)
}
