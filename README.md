# Arithmico

This repository is the monorepo for Arithmico, an accessible web-based scientific calculator and educational software designed specifically for visually impaired and blind students. It provides the necessary computational and accessible UI tools required for learning mathematics and other scientific disciplines.

## Repository Structure

The project is organized as a Cargo workspace, allowing for the coordinated development of multiple Rust packages and infrastructure setups within a single repository. 

The directory structure is organized as follows:

* `/packages/applications/*`: Contains all binary projects (executable applications).
* `/packages/libraries/*`: Contains all shared library crates (e.g., core computation engines, frontend UI components, and state management).
* `/infrastructure`: Contains the CDK infrastructure code for deploying the project.

## Processing Pipeline

```mermaid
flowchart LR
    In([User Input]) --> Lex[Lexer]
    Lex --> Par[Parser]
    Par --> Eval[Evaluator]
    Eval --> Val[Validator]
    Val --> Eng[Engine]
    Eng --> Trans[Transformer]
    Trans --> Ser[Serializer]
    Ser --> Out([Output Rendering])
    
    classDef frontend fill:#1e40af,stroke:#60a5fa,color:#fff;
    classDef core fill:#065f46,stroke:#34d399,color:#fff;
    classDef post fill:#5b21b6,stroke:#a78bfa,color:#fff;
    
    class In,Out frontend;
    class Lex,Par,Eval,Val,Eng core;
    class Trans,Ser post;
```

The processing pipeline dictates how a mathematical expression is handled from the moment it is typed until the final result is presented to the user. It begins with the **User Input** in the Arithmico frontend, capturing the raw string. This text is passed to the **Lexer**, which performs tokenization - breaking the raw text down into a sequence of meaningful symbols such as numbers, operators, and identifiers. The **Parser** then takes these flat tokens and structures them into a hierarchical Node-tree that represents the logical order of operations.

Once the tree is built, the **Evaluator** traverses it to compute the underlying mathematical values. Crucially, all mathematical operator evaluation is handled directly within the evaluator package itself. When functions are invoked during evaluation, **Validation** steps in specifically to check function parameter constraints, ensuring the inputs are valid before proceeding.

The **Engine** provides the broader environment for these computations. It is responsible for session handling and supplies the evaluator with all built-in functions (such as `sin`, `cos`) and constants. After the computation is complete, the raw result moves into the final formatting stages. The **Transformer** normalizes the evaluation results, and the **Serializer** converts this normalized data back into a structured format. Finally, the pipeline concludes with **Output Rendering**, where the Arithmico frontend displays the fully computed result back to the user.

## Development Setup

### Workspace Configuration
This repository relies on Cargo workspaces to manage dependencies and build processes across multiple packages seamlessly.

### Formatting
Code formatting is strictly maintained throughout the repository. We use `rustfmt` for standard Rust code and `leptosfmt` for formatting the frontend components.

### Commit Convention
Commit messages in this repository must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification to ensure a standardized version control history.

## Getting Started

## Getting Started

The easiest way to start developing is by using our provided DevContainer configuration. This ensures you have the exact Rust toolchain, formatting tools (`rustfmt`, `leptosfmt`), and dependencies required for the workspace without needing to configure your local machine.

1. Clone this repository.
2. Open the project folder in an IDE that supports DevContainers (such as Visual Studio Code with the Dev Containers extension).
3. When prompted, select **Reopen in Container**.

Once the container is built and running, your environment is fully configured. You can navigate to any specific package or application directory and immediately use standard Cargo commands to build, run, and test the code.
