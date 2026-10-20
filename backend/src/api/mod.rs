use axum::{routing::get, Json, Router};

use crate::models::{ApiInfo, HealthResponse};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1", get(api_info))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "homelab-control-api".to_string(),
    })
}

async fn api_info() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "homelab-control-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
