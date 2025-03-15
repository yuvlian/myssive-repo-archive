use crate::network::packet;
use database::Arclite;
use hi3_proto::{CgType, PlayerLoginRsp, cmd};

pub async fn handle(user_id: u32, _body: &[u8], _db: &Arclite) -> Vec<u8> {
    let rsp = PlayerLoginRsp {
        region_name: String::from("overseas01"),
        cg_type: CgType::CgSevenChapter.into(),
        region_id: 248,
        login_session_token: 1,
        ..Default::default()
    };

    packet::some_data(user_id, cmd::PLAYER_LOGIN_RSP, rsp)
}
