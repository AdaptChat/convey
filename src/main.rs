use axum::{routing::get, Router};

mod config;
mod error;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:8078".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { drop(tokio::signal::ctrl_c().await) })
        .await
        .unwrap();
}
