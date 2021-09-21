use serde::*;

#[derive(Deserialize)]
pub struct HealthCheckRequest {}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: bool,
}