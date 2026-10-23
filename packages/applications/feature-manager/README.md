# feature-manager

`feature-manager` is a workspace CLI tool for validating fine-grained feature flags across Arithmico Rust packages.

## Features

* **Graph Verification (`verify`)**: Verifies that feature flags originating in the `evaluator` crate are correctly declared and forwarded through the `engine` crate. *(Future verifications for `engine-worker` will be added in upcoming releases.)*
* **CI Enforcement**: Runs in CI pipelines to prevent merging pull requests with disconnected or mismatched feature configurations.

## Usage

Check that all `evaluator` feature flags are properly forwarded by `engine`:

```bash
cargo run -p feature-manager -- verify
```

Returns exit code `0` on success, or lists invalid/missing feature forwards on failure.
