# Abstract Syntax Tree

In the Arithmico parser and evaluator pipeline, mathematical expressions are structured into a hierarchical abstract syntax tree (AST). Because expression structures represent nested operations, the AST is inherently a self-referencing data structure. We needed to choose an underlying AST node representation that allows clean tree traversal while maximizing evaluation performance and developer ergonomics.

## Decision

We decided to represent the AST using a single recursive `Node` enum, where each variant corresponds to a specific AST node type.

We chose the enum-based representation for the following reasons:

* **Static Dispatch and Performance:** Enums allow monomorphized static dispatch across all AST traversals and function calls, eliminating dynamic dispatch runtime overhead during evaluation.

* **Ergonomic Pattern Matching:** Rust's pattern matching on enums provides clean, exhaustive, and type-safe destructuring.

## Alternatives Considered

* **Trait Objects (`Box<dyn Node>`):** Evaluated as an alternative where each node type implements a common node trait. This was rejected because dynamic dispatch (`vtable` lookups) through trait objects introduces noticeble performance bottlenecks during heavy evaluation load. Furthermore, dynamic dispatch reduces the number of optimizations the compiler can apply drastically. Additionally, working with trait objects severely restricts pattern matching ergonomics compared to native Rust enums.

## Consequences

### Positive

* Zero-cost static dispatch for node operations during evaluation.
* Concise, expressive pattern matching across all pipeline stages (evaluator, validator, transformer, ...).
* Exhaustive compiler checks when introducing or modifying AST node variants.

### Negative

* Adding a new node type requires modifying the central `Node` enum definition and handling the new variant across all pattern match sites.
