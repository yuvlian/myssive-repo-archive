mod handlers;
mod middleware;
mod models;
mod router;

use axum::Router;
use middleware::log_requests;
use router::endfield_router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(endfield_router())
        .layer(axum::middleware::from_fn(log_requests));

    let addr: std::net::SocketAddr = "127.0.0.1:21000".parse().unwrap();

    common::init_tracing();
    common::print_mayfield();

    tracing::info!("Listening at: {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind to address");
}
