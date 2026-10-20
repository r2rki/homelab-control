use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}
