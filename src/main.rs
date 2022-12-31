use axum::{
    routing::{get, post},
    Router,
};

mod config;
mod error;
mod routes;

#[tokio::main]
async fn main() {
    drop(dotenv::dotenv());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", post(routes::upload))
        .route(
            "/attachments/:id/*filename",
            get(routes::download).delete(routes::delete),
        );

    axum::Server::bind(&"0.0.0.0:8078".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { drop(tokio::signal::ctrl_c().await) })
        .await
        .unwrap();
}
