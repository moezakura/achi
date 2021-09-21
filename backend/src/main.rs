mod routes;
mod handler;
mod domain;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .nest("/api", routes::api_routes());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
