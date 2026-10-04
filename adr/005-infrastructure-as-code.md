# Adopt AWS CDK for Infrastructure as Code

In Arithmico v1, infrastructure was managed manually without any Infrastructure as Code (IaC) tool. In Arithmico v2, we adopted Terraform to manage cloud resources.

However, as the project evolved, the infrastructure requirements grew more complex. We required a solution capable of handling dynamic environment creation, specifically short-lived review environments created automatically for each pull request and torn down upon merging. Expressing complex conditional deployment logic and managing multi-environment lifecycle orchestration in standard declarative configuration proved rigid and cumbersome.

## Decision

We decided to adopt AWS CDK (Cloud Development Kit) as our standard Infrastructure as Code framework for Arithmico v3.

We chose AWS CDK for the following reasons:

* **High-Level L2/L3 Constructs:** Constructs such as `S3BucketDeployment` and `LoadBalancedFargateService` significantly reduce boilerplate code and encapsulate AWS best practices out of the box.

* **Flexibility in Conditional Logic:** Using a full programming language allows us to easily implement complex dynamic stack generation and conditional deployment rules.

* **Dynamic & Temporary Deployments:** Managing independent stack instances programmatically simplified the automation of short-lived PR review deployments.

## Alternatives Considered

* **No IaC (Arithmico v1 Approach):** Rejected due to lack of reproducibility, absence of version control for infrastructure, and risk of configuration drift.

* **Terraform (Arithmico v2):** Evaluated and replaced. While declarative and multi-cloud friendly, managing highly dynamic, short-lived PR review stacks and complex conditional logic required complex module structures and workaround scripts.

## Consequences

### Positive

* Higher abstraction levels speed up developer velocity when provisioned components like ECS services or S3 assets are needed.

* Automated creation and cleanup of short-lived pull request review deployments in CI/CD workflows.

* Infrastructure logic can be tested and validated using standard programming patterns and tooling.

### Negative

* Tighter coupling to the AWS ecosystem compared to vendor-agnostic IaC tools like Terraform.

* Increased synthesis/build step needed before generating underlying CloudFormation templates.
