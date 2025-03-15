use crate::network::packet;
use database::Arclite;
use hi3_proto::{GetBulletinRsp, cmd};

pub async fn handle(user_id: u32, _body: &[u8], _db: &Arclite) -> Vec<u8> {
    let rsp = GetBulletinRsp {
        is_all: true,
        ..Default::default()
    };

    packet::some_data(user_id, cmd::GET_BULLETIN_RSP, rsp)
}
