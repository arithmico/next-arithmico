# arithmico-e2e

arithmico-e2e is an end-to-end (E2E) testing suite for the Arithmico web application. It is built using Node.js and Playwright to ensure the reliability and correctness of the application.

## Getting Started

### Prerequisites

This project uses a development container (devcontainer) for a consistent development environment. Make sure you have:

- [Visual Studio Code](https://code.visualstudio.com/)
- [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

Open the project in VS Code and reopen it in the devcontainer when prompted.

### Installation

First, install the required dependencies:

```sh
npm ci
npx playwright install --with-deps
```

### Running Tests

To execute the Playwright tests, run:

```sh
CI=true npx playwright test
```

## Technologies Used

- **Testing Framework:** Playwright
- **Runtime:** Node.js
- **Package Manager:** npm
