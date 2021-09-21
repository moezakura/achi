use axum::*;
use serde::*;
use tokio::macros::support::Future;

#[derive(Serialize)]
pub struct HealthStatus {
    status: bool,
}

pub struct HealthCheckController {}

impl HealthCheckController {
    pub fn new() -> HealthCheckController {
        HealthCheckController {}
    }

    pub async fn get(&self) -> extract::Json<HealthStatus> {
        extract::Json(HealthStatus {
            status: true,
        })
    }
}
