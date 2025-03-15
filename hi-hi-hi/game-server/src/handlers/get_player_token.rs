use crate::network::packet;
use database::Arclite;
use hi3_proto::{GetPlayerTokenReq, GetPlayerTokenRsp, cmd, decode};

pub async fn handle(_user_id: u32, body: &[u8], _db: &Arclite) -> Vec<u8> {
    let req = decode::<GetPlayerTokenReq>(body);
    let user_id = req.account_uid.parse::<u32>().unwrap_or(1);

    let rsp = GetPlayerTokenRsp {
        uid: user_id,
        token: req.token,
        account_type: req.account_type,
        account_uid: req.account_uid,
        user_type: 4,
        ..Default::default()
    };

    packet::some_data(user_id, cmd::GET_PLAYER_TOKEN_RSP, rsp)
}
