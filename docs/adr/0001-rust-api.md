# ADR 0001: Rust API

## Decision

Use Rust with Axum and Tokio for the control-plane API.

## Why

- strong type safety
- efficient async I/O
- low runtime overhead
- single-binary deployment option
- strong fit for a long-lived systems-oriented portfolio project

## Constraints

The initial implementation stays modular-monolithic. We will not split the API into microservices unless a concrete need appears.
