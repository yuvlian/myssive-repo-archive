use std::net::SocketAddr;

mod files;
mod handlers;
mod router;

use router::ultina_app;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:21000".parse().unwrap();

    println!("->> listening on {}", addr);

    axum_server::bind(addr)
        .serve(ultina_app().into_make_service())
        .await
        .unwrap()
}
