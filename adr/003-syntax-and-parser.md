# Expression Syntax and Parser Implementation

Arithmico allows users to enter mathematical expressions via a text input interface for evaluation. Designing the expression syntax and parser required balancing several critical constraints:

1. **Backwards Compatibility:** The syntax needed to remain compatible with Arithmico v1 and v2 syntax to ensure existing users and educational materials faced minimal friction.
2. **School Environment Ergonomics:** The grammar must be simple, intuitive, and clean enough for school mathematics contexts.
3. **Braille Accessibility:** Braille output devices (used by blind users) have limited tactile cell space and complex modifier key combinations for rare symbols. Minimizing special character requirements is vital for tactile ergonomics and fast navigation with screen readers.
4. **Locale Adaptability:** Expression syntax must adapt to regional mathematical conventions (e.g., using `,` as a decimal separator and `;` as an argument separator in German vs. `.` and `,` in English).
5. **Translatable Error Messages:** Accessibility compliance requires that syntax errors provide localized, descriptive feedback rather than cryptic or un-translated parser state dumps.

## Decision

We decided to implement a **custom mathematical expression grammar** based on Arithmico v2 (with minor refinements) and build a **standalone parser from scratch in Rust**.

The implementation utilizes a **modified precedence climbing algorithm** to parse expressions efficiently into an Abstract Syntax Tree (AST). It features:
* A decoupled parsing phase separate from expression evaluation.
* Configurable tokenization supporting locale-dependent decimal and parameter separators.
* A custom error tracing architecture that produces structured, translatable error diagnostics.

## Alternatives Considered

We evaluated several existing third-party expression parsers and grammars across the Rust and open-source ecosystems. They were rejected due to one or more of the following issues:

* **Incompatible Grammars:** Syntax deviated too far from previous Arithmico versions, breaking user expectations.
* **Lack of Localization:** Existing parsers hardcoded decimal points (`.`) and function argument separators (`,`), preventing regional customization for German-speaking and international schools.
* **Coupled Evaluation and Parsing:** Many existing libraries combined parsing and evaluation into a single step, preventing AST transformation or custom symbolic manipulation.
* **Non-Translatable Error Diagnostics:** Standard parser error formats output raw string tokens or English-only error messages that could not be internationalized or voiced accessibly by screen readers.
* **Language/Ecosystem Mismatch:** Several viable standalone parser implementations existed only in C/C++ or JavaScript, lacking native Rust bindings suited for WebAssembly compilation.

## Consequences

### Positive
* **Optimal Braille Ergonomics:** Minimizing special characters significantly improves tactile readability and input efficiency for blind users.
* **Full Localization Support:** Supports localized separators (e.g., German vs. English) seamlessly within the same underlying evaluation model.
* **Accessible Error Diagnostics:** Structured error traces allow rendering friendly, localized error messages that screen readers can communicate clearly.
* **High Performance and Low Overhead:** The modified precedence climbing algorithm delivers high-throughput AST generation with minimal heap allocations.
* **Clean Architecture:** Keeping the parser independent from the evaluation engine simplifies testing and enables future symbolic manipulation pipelines.

### Negative / Trade-offs
* **Maintenance Ownership:** Writing and maintaining a custom parser requires ongoing test coverage and maintenance by project contributors.
