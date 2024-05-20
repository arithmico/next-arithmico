# Arithmico

This package is the frontend for the Arithmico application. To build and develop this application, you need to install `trunk`. You can install `trunk` by running the following command in your terminal:
```sh
cargo install --locked trunk
```

## Development

To start the development server and open the application, run:
```sh
trunk serve --open
```

## Build

To build the application for production, run:
```sh
trunk build --release
```

## Feature Selection

To build the application with specific features, run:
```sh
trunk build --release --no-default-features --features "engine/feature-a, engine/feature-b, engine/feature-c"
```
