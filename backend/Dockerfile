FROM rust:1 as builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
RUN mkdir src && printf 'fn main() {}\n' > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/homelab-control-api /app/homelab-control-api
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/app/homelab-control-api"]
