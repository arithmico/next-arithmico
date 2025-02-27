use std::{env, process::ExitCode};

use anyhow::anyhow;
use git2::{Oid, Repository, Sort};
use semver::{Prerelease, Version};

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            println!("Error: {}", error.to_string());
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    let repository = Repository::open_from_env()?;
    let last_version = find_last_version_tag(&repository)?;
    let messages: Vec<String> = list_commits(&repository, &last_version)?;

    let release_mode = if let Ok(value) = env::var("SEMVER_PRERELEASE") {
        if value.to_lowercase() == "true" {
            ReleaseMode::Alpha
        } else {
            ReleaseMode::Release
        }
    } else {
        ReleaseMode::Release
    };

    let new_version =
        create_new_version(&last_version.version, &messages, &release_mode)?;

    println!("v{}", new_version);
    Ok(())
}

enum ReleaseMode {
    Alpha,
    Release,
}

#[derive(PartialEq)]
enum VersionBump {
    Major,
    Minor,
    Patch,
}

fn create_new_version(
    last_version: &Version,
    messages: &[String],
    release_mode: &ReleaseMode,
) -> anyhow::Result<Version> {
    match release_mode {
        ReleaseMode::Alpha => {
            let pre_release_string = last_version
                .pre
                .as_str()
                .strip_prefix("alpha.")
                .ok_or(anyhow!("Invalid pre release"))?;

            let pre_release_number = pre_release_string.parse::<usize>()?;
            let mut version = last_version.clone();
            version.pre =
                Prerelease::new(&format!("alpha.{}", pre_release_number + 1))?;
            Ok(version)
        }
        ReleaseMode::Release => {
            let mut version = last_version.clone();
            let mut version_bump = VersionBump::Patch;

            for message in messages {
                if message.starts_with("feat")
                    && version_bump != VersionBump::Major
                {
                    version_bump = VersionBump::Minor;
                }
                // TODO: detect breaking changes
            }

            match version_bump {
                VersionBump::Major => {
                    version.major += 1;
                }
                VersionBump::Minor => {
                    version.minor += 1;
                }
                VersionBump::Patch => {
                    version.patch += 1;
                }
            }
            Ok(Version::new(version.major, version.minor, version.patch))
        }
    }
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
