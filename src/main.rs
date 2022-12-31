use axum::{
    routing::{get, post},
    Router, extract::DefaultBodyLimit,
};

mod config;
mod error;
mod routes;

#[tokio::main]
async fn main() {
    drop(dotenv::dotenv());

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", post(routes::upload))
        .route(
            "/attachments/:id/*filename",
            get(routes::download).delete(routes::delete),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20));

    axum::Server::bind(&"0.0.0.0:8078".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { drop(tokio::signal::ctrl_c().await) })
        .await
        .unwrap();
}
