use std::process::Command;

use chrono::Utc;

fn main() {
    println!(
        "cargo:rustc-env=SOURCE_DATE_EPOCH={}",
        Utc::now().timestamp()
    );
    let commit_hash = String::from_utf8(
        Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
}
