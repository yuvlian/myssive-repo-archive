use crate::network::packet;
use crate::utils::unix_timestamp_secs;
use database::Arclite;
use hi3_proto::{GetLoginActivityRsp, LoginActivityData, cmd};

pub async fn handle(user_id: u32, _body: &[u8], _db: &Arclite) -> Vec<u8> {
    let rsp = GetLoginActivityRsp {
        login_list: vec![LoginActivityData {
            id: 581,
            login_days: unix_timestamp_secs(),
            accept_time: unix_timestamp_secs(),
            duration_end_time: unix_timestamp_secs() + 1209600,
            ..Default::default()
        }],
        ..Default::default()
    };

    packet::some_data(user_id, cmd::GET_LOGIN_ACTIVITY_RSP, rsp)
}
