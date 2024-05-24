# Engine

This package is used to parse and evaluate mathematical expressions. 
All operations, functions, and constants can be controlled through Cargo features.

## Usage

Add the engine package to your `Cargo.toml`:
```toml
[dependencies]
engine = { path = "./releative/path/to/engine/package" }
```

## Feature Selection

Enable specific features to control the operations, functions, and constants available in the engine. For example:
```sh
cargo build --release --no-default-features --features "your, feature, selection"
```
