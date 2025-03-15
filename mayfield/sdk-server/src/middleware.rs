use axum::http::StatusCode;
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();
    let status = response.status();

    match status {
        StatusCode::OK => {
            tracing::info!("Handled: {} {} -> {} {:.2?}", method, uri, status, duration)
        }
        _ => tracing::info!("Unhandled: {} {}", method, uri),
    }

    response
}
