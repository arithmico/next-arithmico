# Test Automation

Arithmico previously lacked an end-to-end (E2E) testing suite, relying solely on unit tests for critical components. As the application evolved into a complex web calculator, this partial coverage left user journeys, UI features, and accessibility workflows vulnerable to regressions. A comprehensive testing strategy is required to ensure both engine correctness and frontend reliability.

## Decision

We adopted a dual-layer automated testing strategy:

* **Unit Testing:** We mandate unit tests for all critical computation, core logic, parsing, and state management components, explicitly excluding UI components.
* **Playwright E2E Testing:** We use Playwright to validate user interactions, core use cases, and full application features across real browser environments.
* **Automated Accessibility Audits:** We perform automatic accessibility compliance checks using Playwright integrated with `axe-core` to ensure adherence to accessibility standards.

## Alternatives Considered

* **Cypress for E2E:** Evaluated as an alternative framework. It was rejected because Playwright's API better fitted our operational needs, and Cypress lacks native test sharding support.
* **Unit Testing UI Components:** Rejected because testing isolated UI components fails to reliably catch issues with DOM rendering, browser-specific behaviors, and accessibility compliance.

## Consequences

### Positive

* High confidence in critical user journeys and full application features prior to deployment.
* Automated accessibility auditing directly within real browser contexts.
* Fast execution and isolated feedback loops for internal calculation engines via targeted unit tests.
* While not currently in use, Playwright's native sharding support provides a clear scaling path as the E2E test suite grows.

### Negative

* Additional CI pipeline execution time required to run browser-based Playwright suites.
* E2E tests require ongoing maintenance to prevent flakiness as UI flows evolve.
