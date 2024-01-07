use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::path::PathBuf;
use tower_http::services::ServeDir;

/// health_check endpoint
/// serves a 200 OK response with no body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

/// Serves a frontend built with Leptos at /
/// renders the dist when you go to these specific routes
///
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hp", get(health_check))
        .nest_service("/tool", ServeDir::new(PathBuf::from("dist")))
        .nest_service("/about", ServeDir::new(PathBuf::from("dist")))
        .nest_service("/download", ServeDir::new(PathBuf::from("dist")))
        .nest_service("/", ServeDir::new(PathBuf::from("dist")));

    Ok(router.into())
}
