use crate::network::packet;
use database::Arclite;
use database::client_data::insert_client_data;
use hi3_proto::{SetClientDataReq, SetClientDataRsp, cmd, decode};

pub async fn handle(user_id: u32, body: &[u8], db: &Arclite) -> Vec<u8> {
    let req = decode::<SetClientDataReq>(body);

    let rsp = match req.client_data {
        Some(v) => {
            let id = v.id;
            let r#type = v.r#type;
            let data = v.data;

            insert_client_data(db, (id, r#type, data)).await;

            SetClientDataRsp {
                id,
                r#type,
                ..Default::default()
            }
        }
        _ => SetClientDataRsp::default(),
    };

    packet::some_data(user_id, cmd::SET_CLIENT_DATA_RSP, rsp)
}
