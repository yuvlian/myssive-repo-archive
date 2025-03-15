use crate::network::packet;
use database::Arclite;
use hi3_proto::{GetCardProductInfoRsp, cmd};

pub async fn handle(user_id: u32, _body: &[u8], _db: &Arclite) -> Vec<u8> {
    packet::some_data(
        user_id,
        cmd::GET_CARD_PRODUCT_INFO_RSP,
        GetCardProductInfoRsp::default(),
    )
}
