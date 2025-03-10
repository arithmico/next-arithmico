# Arithmico

Arithmico is a web application built using Rust and the Leptos framework.

## Getting Started

### Prerequisites
This project uses a development container (devcontainer) for a consistent development environment. Make sure you have:
- [Visual Studio Code](https://code.visualstudio.com/)
- [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

Open the project in VS Code and reopen it in the devcontainer when prompted.

### Running the Application

To start the application in development mode, run:

```sh
trunk serve
```

This will compile the project and serve it locally.

### Building for Production

To build the application for production, use:

```sh
trunk build --release
```

The optimized output will be available in the `dist` directory.

## Technologies Used

- **Programming Language:** Rust
- **Framework:** Leptos
- **Build Tool:** Trunk
