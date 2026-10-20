use std::{env, net::SocketAddr};

pub fn bind_addr() -> SocketAddr {
    let raw = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    raw.parse()
        .expect("BIND_ADDR must be a valid socket address")
}
