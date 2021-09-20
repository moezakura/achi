use axum::{
    extract::Json,
    handler::get,
    handler::post,
    Router,
    routing::BoxRoute,
};
use serde::{Serialize, Deserialize};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/ping", post(ping))
        .nest("/api", api_routes());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/fuga", get(root))
        .boxed()
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct Ping {
    count: i64,
}

#[derive(Serialize)]
struct Pong {
    count: i64,
}

async fn ping(Json(ping): Json<Ping>) -> Json<Pong> {
    println!("ping count: {}", ping.count);
    Json(Pong {
        count: ping.count + 1,
    })
}