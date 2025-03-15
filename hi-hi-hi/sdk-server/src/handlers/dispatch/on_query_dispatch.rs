use crate::crypto::encrypt_ecb;
use crate::models::{OnQueryDispatchRsp, RegionInfo};

pub async fn on_query_dispatch() -> String {
    let rsp = OnQueryDispatchRsp {
        region_list: vec![RegionInfo {
            dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
            name: String::from("yulian"),
            ..Default::default()
        }],
        ..Default::default()
    };
    encrypt_ecb(&rsp).unwrap_or(serde_json::to_string(&rsp).unwrap_or_default())
}
