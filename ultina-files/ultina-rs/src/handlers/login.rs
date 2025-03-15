use crate::files::{htmx_rsp, ultina_page};
use axum::{
    extract::Json,
    http::{Response, StatusCode},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub u: String,
    pub p: String,
}

pub async fn post(Json(payload): Json<LoginRequest>) -> Response<String> {
    if payload.u == "admin" && payload.p == "password" {
        let content = ultina_page("dashboard.html")
            .await
            .unwrap_or("<h1>Welcome</h1>".to_string());

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(content)
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "text/plain")
            .body("Invalid credentials!".to_string())
            .unwrap()
    }
}

pub async fn get() -> Response<String> {
    let content = htmx_rsp("login.html")
        .await
        .unwrap_or_else(|e| e.to_string());

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(content)
        .unwrap()
}
