use bytes::{Buf, BytesMut};
use smol::Async;
use smol::io::{AsyncReadExt, AsyncWriteExt};
use std::net::{TcpListener, TcpStream};

const HEAD_MAGIC: u32 = 0x9D74C714;
const TAIL_MAGIC_BYTES: [u8; 4] = [0xD7, 0xA1, 0x52, 0xC8];
use sr_proto::cmd::*;

include!("../out/rsp.rs");

fn decode_pk(buffer: &[u8]) -> u16 {
    if buffer.len() < 16 {
        panic!("buf too short");
    }

    let head_magic = u32::from_be_bytes(buffer[0..4].try_into().expect("invalid headmagic slice"));

    if head_magic != HEAD_MAGIC {
        panic!("invalid headmagic val");
    }

    let cmd = u16::from_be_bytes(buffer[4..6].try_into().expect("invalid cmd slice"));

    cmd
}

async fn handle_connection(mut socket: Async<TcpStream>) -> smol::io::Result<()> {
    let mut buffer = BytesMut::with_capacity(1024);
    let mut temp_buffer = [0u8; 1024];

    loop {
        let n = socket.read(&mut temp_buffer).await?;

        if n == 0 {
            return Ok(());
        }

        buffer.extend_from_slice(&temp_buffer[..n]);

        while let Some(position) = buffer
            .windows(4)
            .position(|window| window == TAIL_MAGIC_BYTES)
        {
            if position + 4 <= buffer.len() {
                let complete_message = &buffer[..position + 4];
                let cmd = decode_pk(complete_message);

                match cmd {
                    PLAYER_GET_TOKEN_CS_REQ => socket.write_all(&PLAYER_GET_TOKEN).await?,
                    PLAYER_LOGIN_CS_REQ => socket.write_all(&PLAYER_LOGIN).await?,
                    PLAYER_LOGIN_FINISH_CS_REQ => socket.write_all(&PLAYER_LOGIN_FINISH).await?,
                    GET_AVATAR_DATA_CS_REQ => socket.write_all(&GET_AVATAR_DATA).await?,
                    GET_CUR_LINEUP_DATA_CS_REQ => socket.write_all(&GET_CUR_LINEUP_DATA).await?,
                    GET_MISSION_STATUS_CS_REQ => socket.write_all(&GET_MISSION_STATUS).await?,
                    PLAYER_HEART_BEAT_CS_REQ => socket.write_all(&PLAYER_HEART_BEAT).await?,
                    GET_BASIC_INFO_CS_REQ => socket.write_all(&GET_BASIC_INFO).await?,
                    GET_CUR_SCENE_INFO_CS_REQ => socket.write_all(&GET_CUR_SCENE_INFO).await?,
                    GET_SCENE_MAP_INFO_CS_REQ => socket.write_all(&GET_SCENE_MAP_INFO).await?,
                    GET_BAG_CS_REQ => socket.write_all(&GET_BAG).await?,
                    GET_MULTI_PATH_AVATAR_INFO_CS_REQ => socket.write_all(&GET_MULTI_PATH_AVATAR_INFO).await?,
                    _ => {}
                };

                buffer.advance(position + 4);
            } else {
                break;
            }
        }
    }
}

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 23301))?;
        loop {
            let stream = listener.accept().await?;
            smol::spawn(handle_connection(stream.0)).detach();
        }
    })
}
