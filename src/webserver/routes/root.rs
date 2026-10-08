use axum::{
    routing::{get, post},
    http::{StatusCode},
    Json, Router,
};

pub async fn hello_world() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "Hello, World!")
}

