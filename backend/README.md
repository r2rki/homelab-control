# Rust Control API

Initial Rust/Axum scaffold for the homelab control plane.

## Endpoints

- `GET /health`
- `GET /api/v1`

## Build

```bash
cargo check
cargo test
cargo run
```

The API is intentionally small at this stage. Service adapters, authentication, policy enforcement and persistence will be introduced only after the infrastructure baseline is stable.
