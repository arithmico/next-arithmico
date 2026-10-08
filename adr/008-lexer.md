# Lexer

In previous iterations, Arithmico v3 used a parser built directly on top of the `nom` parser combinator library. This setup presented significant architectural and operational challenges:

* **Performance Overhead:** The `nom`-based implementation suffered from performance bottlenecks during parsing. While caching was introduced to mitigate these issues, it proved to be a fragile and temporary solution.

* **Traceability and Diagnostic Limitations:** The built-in error traits in `nom` could not satisfy our requirements for precise error tracing without requiring extensive, high-maintenance modifications.

* **Grammar and Pipeline Complexity:** Attempting to handle tokenization and grammar parsing simultaneously in a single pass increased parser complexity and hindered robust early error detection.

During the development of our new parsing pipeline, we needed a design that completely eliminates the `nom` dependency, improves execution speed, and delivers clear, localized, and multi-language diagnostic feedback to users.

## Decision

We decided to replace the `nom`-based implementation with a custom, dedicated lexer as a preprocessing stage in the evaluation pipeline. The lexer tokenizes raw text input and produces a flat token stream consumed directly by the parser.

We chose this design for the following reasons:

* **Early Error Detection:** Tokenization errors (such as invalid character sequences or malformed literals) are caught early in the lexing phase.

* **Precise Error Locating and Spans:** Each generated token carries explicit position and span information, allowing for exact error tracking and source location reporting throughout the entire pipeline.

* **High-Quality Localized Errors:** Standardized token error metadata enables rich, user-friendly diagnostic messages that can easily be translated into the user's preferred language.

* **Dependency Removal:** Building an internal lexer allowed us to completely remove `nom` from our codebase.

## Alternatives Considered

* **Combined Parser-Lexer with `nom`:** Evaluated and rejected. Adapting `nom`'s error infrastructure to track source spans and provide localized diagnostics required excessive customizations and hacks. Furthermore, performance problems forced us to rely on caching workaround strategies rather than addressing root parsing speed.

## Consequences

### Positive

* Complete elimination of the `nom` crate dependency.

* Clean separation of concerns between raw character tokenization and structural AST parsing.

* Granular source positioning and span data attached to every token for good diagnostics and error reporting.

* Foundation for high-quality, localizable error messaging.

### Negative

* Additional maintenance responsibility for a handwritten lexer rather than relying on third-party parser generator libraries.
