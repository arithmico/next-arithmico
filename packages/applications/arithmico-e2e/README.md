# arithmico-e2e

The `arithmico-e2e` package ensures the functionality and reliability of the Arithmico application through comprehensive end-to-end testing. By leveraging Playwright, the tests simulate real user interactions and verify that the application behaves as expected.

## Installation

To set up the testing environment, you need to install Playwright and its dependencies. Run the following commands:

```bash
npm ci
npx playwright install --with-deps
```

## Running Tests

To execute the end-to-end tests, use the following command:

```bash
npx playwright test
```
