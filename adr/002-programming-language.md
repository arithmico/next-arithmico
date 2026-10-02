# Programming Language and Framework

Previous iterations of Arithmico were built using web-native technologies:
* **Arithmico v1:** JavaScript with Vue
* **Arithmico v2:** TypeScript with React

While these technologies enabled rapid prototyping and immediate browser deployment, developing a scientific calculator surfaced fundamental limitations in the JavaScript/TypeScript runtime:
1. **Numeric Precision Issues:** JavaScript's single numeric type (`number`, IEEE 754 double-precision float) caused pervasive precision and representation problems when evaluating complex mathematical expressions.
2. **Runtime Error Handling:** Unhandled runtime exceptions and unpredictable casting in JS/TS made expression evaluation fragile and difficult to reason about.
3. **Performance Overhead:** Garbage collection pauses and runtime object overhead impacted computational throughput during heavy numeric transformations and rendering.

For Arithmico v3, we required a platform that supports strict mathematical guarantees, high performance, robust error handling, and target compilation to WebAssembly (WASM) to remain compatible with our web-first strategy (ADR-001).

## Decision

We decided to rewrite Arithmico (v3) in **Rust** as the primary language, compiling to **WebAssembly (WASM)**, and using **Leptos** as the web UI framework.

Key motivations for choosing Rust:
* **Strict Type System and Precision Control:** Rust provides granular control over primitive numeric types (`f64`, `f32`, `i64`, `u128`, etc.) and seamlessly integrates with custom fixed-precision or symbolic math structures, solving the core precision issues of v1 and v2.
* **Explicit Error Handling:** The `Result<T, E>` and `Option<T>` idioms eliminate unexpected `null`/`undefined` runtime errors and force explicit handling of calculation failures (e.g., division by zero, parse errors).
* **Predictable Performance:** No garbage collector (GC) overhead and fine-grained memory management ensure fast, predictable evaluation times for complex expressions.
* **First-Class WASM Support:** Rust features best-in-class compilation to target `wasm32-unknown-unknown`, allowing heavy computation to run near-native speeds inside browser engines.
* **Superior Tooling and Ecosystem:** High-quality package management (`cargo`), integrated testing, comprehensive documentation generators, and modern developer tooling.
* **Leptos Framework:** Leptos enables a high-performance, fine-grained reactive UI paradigm built natively in Rust that compiles directly to WASM without requiring a heavy virtual DOM layer.

## Alternatives Considered

### 1. Refactor and Optimize Existing TypeScript Codebase (v2)
* **Pros:** Preserves existing codebase and developer familiarity with React/TypeScript ecosystem.
* **Cons:** Does not address underlying JavaScript runtime limitations (double-precision floating point limits, dynamic runtime overhead, implicit coercions). We would spend continuous effort fighting runtime behavior rather than solving mathematical domain problems.

### 2. Traditional Systems Languages (C or C++) compiled to WASM
* **Pros:** Excellent performance, explicit memory control, mature math libraries.
* **Cons:**
  * Poor WebAssembly ecosystem tooling and complex build toolchains compared to `cargo`.
  * Manual memory management increases source potential for memory safety bugs (buffer overflows, use-after-free).
  * Lack of modern, idiomatic UI frameworks targetable directly to WASM compared to Rust options like Leptos.

## Consequences

### Positive
* **Mathematical Correctness:** Strict typing and explicit numeric primitive control eliminate whole classes of floating-point and type coercion bugs.
* **System Stability:** Explicit error handling ensures all evaluation edge cases are handled at compile time.
* **Performance:** Execution speeds are significantly faster than equivalent JS/TS implementations, with lower memory consumption.
* **Unified Technology Stack:** Core calculation logic, parsing pipelines, and UI reactivity are written in a single language (Rust) and compiled into a unified WASM artifact.

### Negative / Trade-offs
* **Steeper Learning Curve:** Rust's borrow checker and type system require more up-front mental model adjustment for new open-source contributors compared to JavaScript/TypeScript.
* **Build Times:** Cargo compilation to WASM takes longer than typical TypeScript bundling steps.
