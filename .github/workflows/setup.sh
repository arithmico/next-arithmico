#!/usr/bin/env bash

rustup target add wasm32-unknown-unknown
cargo binstall -y --force trunk
cargo install cssbundler --path packages/applications/cssbundler
