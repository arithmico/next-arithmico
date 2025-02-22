# install tailwind
npx -y playwright install --with-deps

# install rust toolchain and utilities
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install leptosfmt
cargo install cargo-watch