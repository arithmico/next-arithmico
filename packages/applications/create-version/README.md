# create\_version

`create_version` is a CLI tool written in Rust that determines the next version number based on the Git history, following Semantic Versioning (SemVer). It is designed to be used in CI pipelines for automatic version generation.

## Features

- Analyzes Git commit history to determine the next version number.
- Follows Semantic Versioning (SemVer) rules.
- Easily integrates into CI/CD pipelines.

## Installation

You can install `create_version` using Cargo:

```sh
cargo install create_version
```

## Usage

Run the tool in a Git repository to determine the next version:

```sh
create_version
```

### Example Output

```sh
v1.2.3
```

## Integration in CI

To use `create_version` in a CI pipeline, simply run it as part of your versioning step:

```sh
version=$(create_version)
echo "Next version: $version"
```

This version can then be used for tagging or publishing artifacts.

## Technologies Used

- **Programming Language:** Rust
- **Versioning Standard:** Semantic Versioning (SemVer)
- **VCS:** Git
