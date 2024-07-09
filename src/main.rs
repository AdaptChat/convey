#![feature(string_remove_matches)]

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use config::ASSETS_PATH;
use tower_http::services::ServeDir;

mod config;
mod error;
mod routes;
mod storage;

#[tokio::main]
async fn main() {
    drop(dotenv::dotenv());

    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "info"); }
    }

    pretty_env_logger::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/attachments", post(routes::upload))
        .route(
            "/attachments/*filename",
            get(routes::download).delete(routes::delete),
        )
        .route(
            "/avatars/*filename",
            post(routes::upload_avatar).get(routes::download_avatar),
        )
        .route(
            "/icons/:id",
            post(routes::upload_avatar).get(routes::download_avatar),
        )
        .route(
            "/banners/:id",
            post(routes::upload_avatar).get(routes::download_avatar),
        )
        .route(
            "/emojis/:id",
            post(routes::upload_emoji)
                .get(routes::download_emoji)
                .delete(routes::delete_emoji),
        )
        .fallback_service(ServeDir::new(&*ASSETS_PATH))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20));

    axum::Server::bind(&"0.0.0.0:8078".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { drop(tokio::signal::ctrl_c().await) })
        .await
        .unwrap();
}
