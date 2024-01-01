use axum::{routing::get, Router, response::{Response, Html}, http::header, body::{Bytes, Body}};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    Ok(router.into())
}

