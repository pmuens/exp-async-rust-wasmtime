# `exp-async-rust-wasmtime`

Experiment: Asynchronous Rust via Wasmtime.

## Useful Commands

```sh
# Add WASM target
rustup target add wasm32-unknown-unknown

# Build WASM code
cargo build --manifest-path ./wasm/Cargo.toml --target wasm32-unknown-unknown

# Run tests
cargo test
```
