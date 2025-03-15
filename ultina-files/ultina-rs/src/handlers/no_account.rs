use crate::files::htmx_rsp;
use axum::http::{Response, StatusCode};

pub async fn get() -> Response<String> {
    let content = htmx_rsp("no-account.html")
        .await
        .unwrap_or_else(|e| e.to_string());

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(content)
        .unwrap()
}
