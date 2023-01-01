use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
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
        .route("/attachments", post(routes::upload))
        .route("/avatars/:user_id", post(routes::upload_avatar))
        .route(
            "/attachments/:id/*filename",
            get(routes::download).delete(routes::delete),
        )
        .route("/avatars/:user_id/*id", get(routes::download_avatar))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20));

    axum::Server::bind(&"0.0.0.0:8078".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { drop(tokio::signal::ctrl_c().await) })
        .await
        .unwrap();
}
