#!/usr/bin/env bash

sudo apt update
sudo apt install -y \
    pkg-config \
    libdbus-1-dev \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# install tailwind
npx -y playwright install --with-deps

# install rust toolchain and utilities
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rustfmt
cargo install trunk@0.22.0 --locked
cargo install leptosfmt
cargo install cargo-watch
cargo install tauri-cli --version "^2" --locked

