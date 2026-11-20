# Implement published numerical algorithms instead of using `statrs`

## Context

Arithmico requires numerical algorithms for statistical distributions, special functions, root finding, numerical differentiation, and other mathematical functionality.

These algorithms form part of the numerical foundation of Arithmico and therefore need to provide predictable and verifiable numerical behavior.

## Requirements

Numerical implementations should provide sufficient precision across the supported input ranges and handle relevant edge cases reliably.

Where established algorithms exist, their implementation should be verifiable against:

* original publications or authoritative reference implementations,
* published reference values,
* upstream or independently verified test cases.

The mathematical implementations should also remain independent of Arithmico's AST and evaluator.

## Decision

Required numerical algorithms are implemented in `math-utils` based on established and publicly documented algorithms and reference implementations.

Where applicable, each implementation should reference its underlying algorithm and include corresponding reference tests.

The goal is not to develop new numerical methods, but to implement established algorithms while retaining explicit control over algorithm selection, precision, and edge-case behavior.

Focused lower-level dependencies such as `nalgebra` may still be used where appropriate.

## Alternative Considerations

### Use `statrs`

`statrs` provides many statistical distributions and special functions and originally started as a Rust port of functionality from the C# Math.NET library.

Using `statrs` would reduce the amount of numerical code maintained by Arithmico. However, its inherited and library-specific implementation choices do not provide the required level of control over algorithm selection, numerical precision, verification, and edge cases.

For these reasons, `statrs` is not used as the implementation layer for Arithmico's numerical functionality.

### Implement new algorithms specifically for Arithmico

Developing novel numerical algorithms was rejected because established and well-studied methods should be preferred whenever suitable published algorithms are available.

## Consequences

Numerical algorithms are maintained in `math-utils`, while higher-level concerns such as evaluation, `Node`, translations, and argument validation remain outside this crate.

### Positive

* Explicit control over numerical algorithms and precision.
* Implementations can be traced to published sources.
* Results can be verified against authoritative reference values and implementations.
* Numerical behavior is independent of changes in broad external statistics libraries.

### Negative

* Arithmico is responsible for maintaining and testing the implementations.
* Numerical stability and edge cases require careful review.
* Implementing and verifying established algorithms requires more development effort than wrapping an existing library.