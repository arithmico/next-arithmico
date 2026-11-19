# Modernize Configuration and Build Infrastructure with Serverless Rust Architecture

Previously, Arithmico's configuration and build orchestration service operated as a Node.js web service deployed on AWS ECS Fargate behind an Application Load Balancer (ALB). This architecture relied on several external and stateful components:

* **Persistence & Queuing:** MongoDB (hosted on the MongoDB Atlas free tier) for storing metadata and Redis with BullMQ (hosted on the Redis Cloud free tier) for managing build job queues.
* **Authentication:** GitHub Personal Access Tokens (PATs) to trigger build workflows and read feature metadata, alongside custom-issued JWTs to authenticate incoming feature metadata reports.

As Arithmico transitioned toward a unified, pure Rust monorepo, maintaining a separate Node.js service introduced ecosystem fragmentation. Furthermore, running always-on ECS Fargate tasks and an ALB incurred unnecessary baseline operational costs and complexity, relying on third-party free tiers (MongoDB Atlas and Redis Cloud) introduced external operational constraints, and managing PAT rotation alongside custom JWT issuance created avoidable security and maintenance overhead.

## Decision

We decided to migrate the configuration and build infrastructure to a serverless, AWS-native, Rust-based architecture:

* **Language and Compute (Rust on AWS Lambda + API Gateway):** Replace Node.js, ECS Fargate, and ALB with Rust binaries running on AWS Lambda behind Amazon API Gateway. Writing the service in Rust integrates it directly into our Cargo workspace and pure Rust architecture, while serverless compute eliminates idle infrastructure costs and container orchestration overhead.
* **GitHub Authentication (GitHub App):** Authenticate as an installed GitHub App instead of using PATs, completely eliminating manual PAT rotation and scoped user-token management.
* **Metadata Reporting Auth (GitHub OIDC Tokens):** Replace custom-issued JWTs with GitHub OpenID Connect (OIDC) ID tokens when reporting feature metadata from release workflows, reducing cryptographic complexity while improving security through short-lived, verifiable identity claims.
* **Database (Amazon DynamoDB):** Replace MongoDB with Amazon DynamoDB. Because our access patterns do not require complex server-side aggregations, DynamoDB satisfies our persistence needs while removing reliance on the MongoDB Atlas free tier and minimizing operational maintenance.
* **Job Queuing (Amazon SQS):** Replace Redis and BullMQ with Amazon SQS to buffer and dispatch build jobs, eliminating reliance on the Redis Cloud free tier and simplifying queue operations.

## Alternatives Considered

* **Retaining the Node.js + Fargate + Third-Party Free-Tier Stack:** Rejected due to continuous baseline compute costs, operational overhead of managing containers and load balancers, external dependency risks from third-party free tiers, manual PAT rotation, and the friction of maintaining Node.js tooling inside a Rust monorepo.
* **Self-Hosting MongoDB and Redis on AWS (EC2/ECS):** Rejected because provisioning, patching, and backing up self-hosted stateful databases and brokers would substantially increase operational complexity and cost compared to managed serverless services.

## Consequences

### Positive

* **Lower Operational Cost and Complexity:** Consolidating onto serverless AWS primitives (Lambda, API Gateway, DynamoDB, SQS) removes idle compute charges, eliminates external free-tier dependencies, and reduces infrastructure maintenance.
* **Strengthened Security Posture:** Switching to GitHub App authentication removes long-lived PATs and their rotation burden, while GitHub OIDC tokens eliminate custom token issuance logic.
* **Unified Monorepo Experience:** Implementing the service in Rust allows it to live natively within `/packages/applications/*`, sharing workspace dependencies, formatting rules, and CI workflows with the rest of the codebase.

### Negative

* **Upfront Access-Pattern Modeling:** Moving from MongoDB to DynamoDB trades flexible ad-hoc querying and aggregations for strict key-based access patterns, requiring careful schema design (e.g., Single-Table Design and secondary indexes).
* **Increased AWS Coupling:** Replacing portable technologies (Node.js containers, MongoDB, Redis) with AWS-specific managed services increases vendor coupling, though this aligns with our existing AWS CDK infrastructure strategy.
* **Cold-Start Latency:** Lambda functions can experience cold starts compared to always-on ECS Fargate containers, though Rust's small binary size and fast runtime initialization keep cold-start overhead minimal.
