# Contributing to Arithmico

Welcome and thank you so much for your interest in contributing to Arithmico! We are thrilled to have you here. As a project dedicated to providing an accessible and high quality scientific computing environment for visually impaired and blind students, we deeply value our community and hold our codebase to rigorous standards.

To maintain the integrity, security, and accessibility of Arithmico, we have established specific guidelines for how code is written and submitted. We want to ensure that every contribution makes a meaningful impact!

## 🛑 Strictly Human Contributions Only

Arithmico relies on precise mathematical evaluations and carefully crafted accessibility features. Because of the exactness required across our entire architecture, we have a **strict policy forbidding automated, agent driven, or vibe coded contributions**.

We enforce this policy for two primary reasons:
1. **Legal and Licensing Risks:** AI generated content introduces significant copyright risks and potential licensing issues that we cannot accept.
2. **Maintainer Bandwidth:** Our maintainers have a very limited amount of time. We must focus our review bandwidth solely on intentional and human driven contributions rather than spending it on auditing or debugging auto generated code.

### 1. No Autonomous Agents
We do not accept pull requests generated, submitted, or managed by autonomous AI coding agents. All contributions must be driven by a human developer who can actively participate in code review, articulate their design decisions, and engage in meaningful discussion.

### 2. No Vibe Coding
Vibe coding is the practice of continuously pasting LLM generated code until the compiler stops complaining, without fundamentally understanding the underlying logic. This practice is strictly forbidden.

* Whether you are working on the underlying computational logic or the user facing accessibility features, you must deeply understand the implications of your changes.
* You must be aware of how your modifications fit into the broader system architecture and evaluation lifecycle.

While using AI assistants as a learning tool or syntax helper is completely fine, **you take full human responsibility for every line of code you submit**. You must be able to fully explain your implementation choices during the review process. Code that looks auto generated and lacks human intent will be rejected.

## Development Setup

We utilize a DevContainer setup to ensure a consistent and seamless environment across all contributors.

1. Clone the repository.
2. Open the project in an IDE that supports DevContainers.
3. Select **Reopen in Container**.

This will automatically provision the correct Rust toolchain, formatting tools, and all necessary dependencies without requiring local machine configuration.

## Coding Standards

* **Formatting:** All standard Rust code must be formatted with `rustfmt`. Frontend leptos code must be formatted with `leptosfmt`.
* **Workspace:** Arithmico is a Cargo workspace. Ensure you are testing and building from the correct package directory.

## Commit Convention

We strictly follow the Conventional Commits specification. This ensures a clean version history and allows us to track changes cleanly across the workspace.
* Examples: `feat: add hyperbolic sine function`, `fix: resolve syntax tree traversal bug`, `docs: update contributing guidelines`.

## Pull Request Process

1. Fork the repository and create your feature branch from main.
2. Write your code, ensuring it is fully understood, intentional, and tested.
3. Commit your changes using Conventional Commits.
4. Open a Pull Request with a clear description of the problem solved and your technical approach.
5. Be prepared to answer architectural and logical questions about your contribution during the review.

We cannot wait to see what you build! We look forward to your thoughtful, deliberate, and entirely human contributions!