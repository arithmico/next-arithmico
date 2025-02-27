use git2::{Oid, Repository, Sort};
use semver::Version;

fn main() {
    match run() {
        Ok(_) => (),
        Err(error) => println!("Error: {}", error.to_string()),
    }
}

fn run() -> anyhow::Result<()> {
    let repository = Repository::open_from_env()?;
    let last_version = find_last_version_tag(&repository)?;
    let messages: Vec<String> = list_commits(&repository, &last_version)?;
    println!("messages = {:#?}", messages);
    Ok(())
}

#[derive(Debug, Clone)]
struct VersionTag {
    version: Version,
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
                    versions.push(VersionTag { oid, version });
                }
            }
            true
        })
        .expect("all tags checked");

    versions.sort_by(|a, b| a.version.cmp(&b.version));
    let last = versions.last().cloned().expect("last");

    Ok(last)
}

fn list_commits(
    repository: &Repository,
    version_tag: &VersionTag,
) -> anyhow::Result<Vec<String>> {
    let mut revwalk = repository.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;
    let mut commit_messages = Vec::new();

    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repository.find_commit(commit_id)?;
        if commit_id == version_tag.oid {
            break;
        }
        if let Some(commit_message) = commit.message() {
            commit_messages.push(commit_message.to_string());
        }
    }

    Ok(commit_messages)
}
