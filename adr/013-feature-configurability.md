# Feature Configurability

Arithmico relies on fine-grained feature flags to control its mathematical capabilities, with every datatype, operation, function, and constant being configurable. These feature flags originate from different crates within the workspace; for example, the `evaluator` crate defines feature flags for datatypes and operations, while the `engine` crate handles the flags for functions and constants. To expose these capabilities, upstream crates in the workspace must explicitly declare and forward all downstream feature flags.

## Decision

We decided to make every datatype, operation, function, and constant configurable via Cargo features.

* To create customized versions of Arithmico with specific feature sets, developers compile the `engine-worker` crate with default features disabled and explicitly set the required features.
* All downstream features (such as those from `engine` and `evaluator`) are forwarded via the `engine-worker` crate.
* To maintain structural integrity and prevent merging invalid configurations, we validate that all features are correctly forwarded and enabled by default as an automated step in the CI pipeline.

## Alternatives Considered

We considered no alternatives since utilizing Cargo features is a standard best practice in the Rust ecosystem for conditional compilation and modular feature management.

## Consequences

### Positive

* Provides the flexibility to compile minimal, optimized builds by only enabling the required mathematical features.
* Validating feature flags in the CI pipeline safely prevents pull requests with mismatched or disconnected feature wiring from being merged.

### Negative

* Upstream crates like `engine-worker` must duplicate and explicitly forward all downstream feature flags, which requires additional synchronization and boilerplate.
