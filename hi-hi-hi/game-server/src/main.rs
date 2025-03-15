use cfg_utils::server::ServerConfig;
use tokio::net::TcpListener;

mod handlers;
mod network;
mod utils;
use network::connection;
// use resource::Bh3Data;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Bh3Data::print_json_test("AbilitySpecial");

    let addr: std::net::SocketAddr = {
        let cfg = ServerConfig::from_file("./_cfg/server.toml");
        format!("{}:{}", cfg.game_server_host, cfg.game_server_port)
            .parse()
            .expect("failed parsing to socketaddr")
    };

    println!("->> game-server listening at {}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind to address");

    let db = database::init_sqlite_pool().await;

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .expect("failed to accept connection");

        println!("->> new connection: {}", addr);

        let db_clone = db.clone();

        tokio::spawn(async move {
            if let Err(e) = connection::handle(stream, addr, db_clone).await {
                println!("->> connection error: {}", e);
            }
        });
    }
}
