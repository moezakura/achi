use axum::{
    Router,
    routing::BoxRoute,
    handler::*,
};
use super::handler;

pub fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/health_check", get(|| async {
            let handler = handler::health_check::HealthCheckController::new();
            handler.get().await
        }))
        .boxed()
}
