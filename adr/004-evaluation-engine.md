# Custom Evaluation Engine with Modular Feature Configuration

After parsing an input expression into an Abstract Syntax Tree (AST) using our custom parser (ADR-003), Arithmico requires an evaluation engine to execute mathematical transformations, compute numerical results, and handle symbolic operations.

The evaluation engine must operate under strict non-functional and domain requirements:
1. **Seamless Diagnostic Integration:** Evaluation failures (e.g., division by zero, domain errors, type mismatches) must produce structured error traces that map directly back to AST nodes and token positions.
2. **Localized Engine Diagnostics:** Accessibility standards require that all evaluation error messages, warning conditions, and result metadata be fully translatable into the user's active language.
3. **Regulatory Exam Configurability:** Schools, universities, federal states, and national educational bodies (e.g., IQB guidelines in Germany) impose strict and often contradictory regulations on allowed mathematical tools during exams. For instance, specific examination boards permit numeric calculations but explicitly ban symbolic differentiation, matrix inversions, or specific statistical distributions.

## Decision

We decided to build a **custom evaluation engine from scratch in Rust** tailored specifically to the Arithmico AST and architecture.

Key design highlights of the engine include:
* **Feature Flags:** Every operator, function, constant, and data type (e.g., matrices, numbers, sums, products) can be toggled on or off via feature flags.
* **Tailored Variant Generation:** Enables building custom application variants (e.g., restricted exam builds vs. full-featured scientific calculator builds) without modifying core evaluation logic.
* **Localized Error Reporting:** Evaluator errors return structured diagnostic payloads rather than static strings, allowing the UI layer to render fully localized, screen-reader-friendly error messages.
* **Direct AST & Token Mapping:** Evaluation errors preserve token location context from the custom parser, highlighting exact expression ranges where evaluation failed.

## Alternatives Considered

We evaluated existing third-party math evaluation and Computer Algebra System (CAS) libraries across the Rust, C/C++, and JavaScript ecosystems. They were rejected due to the following critical deal-breakers:

* **Lack of Granular Feature Togglability:** Existing engines operate as monolithic "all-or-nothing" packages. None provided a way to disable individual functions, operators, or data types to satisfy specific regional examination mandates.
* **Opaque or Un-localized Error Systems:** Existing libraries return hardcoded English error strings or generic fault codes that cannot be translated or voiced accessibly to blind users using assistive technology.
* **Tight Coupling to Non-Standard ASTs:** Third-party evaluation engines expect their own AST structures, requiring conversion layers that stripped out source token locations needed for precise error reporting.

## Consequences

### Positive
* **Exam Compliance:** Provides complete control over which mathematical features are available, allowing Arithmico to fulfill various state and institutional exam requirements cleanly.
* **Accessible and Localized Diagnostics:** Engine errors integrate natively with Arithmico’s internationalization system, giving visually impaired users clear, localized feedback on mathematical runtime errors.
* **Architectural Synergy:** Direct AST evaluation avoids translation layers between parser and engine, yielding fast execution and clean memory layouts.
* **Modular Codebase:** Adding or modifying mathematical functions is isolated to small, self-contained modules.

### Negative / Trade-offs
* **Implementation Effort:** Implementing mathematical algorithms, numerical edge-case handling, and domain checks requires significant domain knowledge and ongoing maintainer investment.
* **Validation and Testing Overhead:** Every feature flag combination and custom variant requires thorough unit testing to prevent unexpected evaluation edge cases in restricted builds.
