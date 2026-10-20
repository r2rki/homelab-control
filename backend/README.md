# Backend

Rust API built with Axum and Tokio.

## Current endpoints

- `GET /health`
- `GET /api/v1`

The API is intentionally small in Phase 0/1. Service integrations will be added behind the API layer rather than exposing third-party credentials to the web client.

## Local development

```bash
cargo fmt --all
cargo check --all-targets
cargo test --all-targets
cargo run
```
