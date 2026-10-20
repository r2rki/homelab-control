use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

#[derive(Debug, Serialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
}
