use axum::{routing::get, Json, Router};
use crate::models::{ApiInfo, HealthResponse};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1", get(info))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn info() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "homelab-control-api",
        version: env!("CARGO_PKG_VERSION"),
        status: "scaffold",
    })
}
