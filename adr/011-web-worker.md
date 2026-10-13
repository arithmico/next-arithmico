# Offload Parsing and Evaluation to Web Worker

The Arithmico evaluation pipeline processes user input by tokenizing raw text (Lexer), structuring tokens into a node tree (Parser), and computing values (Evaluator). In previous iterations, the core execution engine ran directly on the browser's main thread alongside the Leptos web frontend.

As expression complexity increased, performing tokenization, parsing, and evaluation synchronously on the main thread caused noticeable UI freezes and unresponsiveness (especially in debug builds). To maintain a fluid user experience, these expensive computational steps were moved off the UI thread.

## Decision

We decided to offload the parsing and evaluation stages of the execution pipeline to a dedicated Web Worker running as a WebAssembly binary.

Key aspects of this architecture include:

* **Frontend Handling:** The frontend remains responsible for user input capture, rendering output, and handling serialization tasks.

* **Dedicated Binary Crate (`engine-worker`):** A standalone binary crate compiled to WebAssembly that serves as the background worker running the lexer, parser, and evaluator pipeline.

* **Shared Protocol Crate (`engine-protocol`):** A shared library defining the strongly typed message protocol used for communication across the client/worker boundary.

* **Envelope Protocol & Correlation IDs:** Communication between the main thread and the worker utilizes an envelope pattern (`CommandEnvelope` and `EventEnvelope`) with correlation IDs to map asynchronous commands to emitted events.

## Alternatives Considered

* **Synchronous Main-Thread Execution:** Running evaluation directly on the main thread. Rejected due to main-thread blocking during expensive mathematical calculations, which causes UI freezes and negatively impacts user accessibility.

* **Shared Array Buffers / Atomics:** Utilizing shared memory across threads. Rejected due to added cross-origin isolation requirements (COOP/COEP headers) and increased code complexity compared to structured message passing via postMessage.

## Consequences

### Positive

* **Non-Blocking UI:** Heavy parsing and evaluation workloads are isolated in a background thread, preventing UI freezes and ensuring a smooth, responsive user interface.

* **Type-Safe Messaging Boundary:** The dedicated `engine-protocol` crate ensures static verification and compile-time type safety across the client-worker communication boundary.

### Negative

* **Asynchronous Communication Overhead:** Message passing and structured serialization via `serde` introduce a minor latency overhead per operation compared to direct in-memory function calls.

* **Infrastructure Complexity:** Managing multiple crates (`engine-worker`, `engine-protocol`) and correlation ID tracking adds boilerplate.
