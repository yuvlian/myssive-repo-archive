use crate::network::packet;
use crate::network::router;
use bytes::{Buf, BytesMut};
use database::Arclite;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle(mut stream: TcpStream, addr: SocketAddr, db: Arclite) -> tokio::io::Result<()> {
    let mut buffer = BytesMut::with_capacity(1024);
    let mut temp_buffer = [0u8; 1024];

    loop {
        let n = stream
            .read(&mut temp_buffer)
            .await
            .expect("failed to read stream");

        if n == 0 {
            println!("->> {} closed connection", addr);
            return Ok(());
        }

        buffer.extend_from_slice(&temp_buffer[..n]);

        while let Some(position) = buffer
            .windows(4)
            .position(|window| window == [0x89, 0xAB, 0xCD, 0xEF])
        {
            if position + 4 <= buffer.len() {
                let (cmd_id, user_id, body) = packet::decode(&buffer[..position + 4]);

                println!("->> {}", cmd_id);

                let response = router::ping_pong((cmd_id, user_id, body), &db).await;

                if !response.is_empty() {
                    stream
                        .write_all(&response)
                        .await
                        .expect("failed to write stream");

                    println!("<<- ✓✓");
                }
                buffer.advance(position + 4);
            } else {
                break;
            }
        }
    }
}
