use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::path::PathBuf;
use tower_http::services::ServeDir;

// health_check endpoint
// serves a 200 OK response with no body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// serves a frontend built with Yew at /
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hp", get(health_check))
        .nest_service("/", ServeDir::new(PathBuf::from("dist")));

    Ok(router.into())
}