use crate::crypto::encrypt_ecb;
use crate::models::{GateAddr, GatewayExt, OnQueryGatewayRsp};

pub async fn on_query_gateway() -> String {
    let rsp = OnQueryGatewayRsp {
        account_url: String::from("http://127.0.0.1:21000/account"),
        account_url_backup: String::from("http://127.0.0.1:21000/account"),
        ext: serde_json::json!("{}"),
        gameserver: GateAddr {
            ip: String::from("127.0.0.1"),
            port: 23301,
        },
        gateway: GateAddr {
            ip: String::from("127.0.0.1"),
            port: 23301,
        },
        is_data_ready: true,
        get_hotpatch_manifest: serde_json::json!("{}"),
        oaserver_url: String::from("http://127.0.0.1:21000/oaserver"),
        region_name: String::from("yulian"),
        server_cur_time: {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time error")
                .as_millis() as u64
        },
        server_cur_timezone: 8,
        server_ext: GatewayExt {
            cdkey_url: String::from("http://127.0.0.1:21000/common/"),
            is_official: String::from("1"),
            mihoyo_sdk_env: String::from("0"),
            use_account_web_url: String::from("1"),
        },
        asset_bundle_url_list: vec![
            String::from("https://bundle-qcloud.bh3.com/asset_bundle/android01/1.0"),
            String::from("https://bundle.bh3.com/asset_bundle/android01/1.0"),
        ],
        ex_resource_url_list: vec![
            String::from("bundle-qcloud.bh3.com/tmp/Original"),
            String::from("bundle.bh3.com/tmp/Original"),
        ],
        ..Default::default()
    };
    encrypt_ecb(&rsp).unwrap_or(serde_json::to_string(&rsp).unwrap_or_default())
}
