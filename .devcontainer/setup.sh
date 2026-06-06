#!/usr/bin/env bash

sudo apt update
sudo apt install -y pkg-config

# install tailwind
npx -y playwright install --with-deps

# install rust toolchain and utilities
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rustfmt
cargo install trunk --locked
cargo install leptosfmt
cargo install cargo-watch
cargo install cssbundler --path packages/applications/cssbundler
