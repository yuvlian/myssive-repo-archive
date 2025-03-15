use crate::network::packet;
use database::Arclite;
use database::client_data::get_all_client_data;
use hi3_proto::{ClientData, GetClientDataReq, GetClientDataRsp, cmd, decode};

pub async fn handle(user_id: u32, body: &[u8], db: &Arclite) -> Vec<u8> {
    let req = decode::<GetClientDataReq>(body);
    let client_data_tuples = get_all_client_data(db).await;

    let client_data_list = client_data_tuples
        .iter()
        .map(|d| ClientData {
            id: d.1,
            r#type: d.2,
            data: d.3.to_vec(),
        })
        .collect();

    let rsp = GetClientDataRsp {
        id: req.id,
        r#type: req.r#type,
        client_data_list,
        ..Default::default()
    };

    packet::some_data(user_id, cmd::GET_CLIENT_DATA_RSP, rsp)
}
