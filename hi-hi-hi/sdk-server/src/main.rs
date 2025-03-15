mod crypto;
mod handlers;
mod middleware;
mod models;
mod router;

use axum::Router;
use cfg_utils::server::ServerConfig;
use middleware::log_requests;
use router::{auth_router, config_router, dispatch_router};

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = {
        let cfg = ServerConfig::from_file("./_cfg/server.toml");
        format!("{}:{}", cfg.sdk_server_host, cfg.sdk_server_port)
            .parse()
            .expect("failed parsing to socketaddr")
    };

    let app = Router::new()
        .merge(auth_router())
        .merge(config_router())
        .merge(dispatch_router())
        .layer(axum::middleware::from_fn(log_requests));

    println!("->> sdk-server listening at {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind to address");
}
