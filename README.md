# PINGU(ping you)

Simple rust ping service to test network connectivity.

## Rust
stable-aarch64-apple-darwin (default)
rustc 1.76.0 (07dca489a 2024-02-04). 

## Build
```sh
cargo build
```

## Start
```sh
PORT=5001 cargo run
```

## Methods 
- ping : Send a GET request to /ping and it return text with 'pong!'.
