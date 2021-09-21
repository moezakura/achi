use axum::*;

use crate::domain::model::handler::health_check::*;

pub struct HealthCheckController {}

impl HealthCheckController {
    pub fn new() -> HealthCheckController {
        HealthCheckController {}
    }

    pub async fn get(&self) -> extract::Json<HealthCheckResponse> {
        extract::Json(HealthCheckResponse {
            status: true,
        })
    }
}
