run-backend:
    cargo build --target wasm32-unknown-unknown -p simple-plugin
    cargo run -p backend
