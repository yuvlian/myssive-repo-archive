use axum::{extract::Request, middleware::Next, response::Response};

pub async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    println!("->> {} {}", method, uri);

    let response = next.run(req).await;
    let status = response.status();

    println!("<<- {}", status);

    response
}
