use sr_proto::MsgTrait;
use std::fs;
use std::path::Path;

fn main() {
    let dispatch_data = rbase64::encode(
        &sr_proto::pb::GlobalDispatchData {
            retcode: 0,
            msg: String::from("OK"),
            top_sever_region_name: String::from("m"),
            server_list: vec![sr_proto::pb::ServerData {
                title: String::from("m"),
                name: String::from("m"),
                display_name: String::from("m"),
                dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
                msg: String::from("OK"),
                env_type: String::from("2"),
            }],
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let gateway_data = rbase64::encode(
        &sr_proto::pb::Gateserver {
            lua_url: String::from(""),
            lua_version: String::from(""),
            ex_resource_url: String::from(""),
            asset_bundle_url: String::from(""),
            use_tcp: true,
            ip: String::from("127.0.0.1"),
            port: 23301,
            unk1: true,
            unk2: true,
            unk3: true,
            unk4: true,
            unk5: true,
            unk6: true,
            unk7: true,
            unk9: true,
            unk10: true,
            unk11: true,
            unk12: true,
            unk13: true,
            unk14: true,
            unk15: true,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let dest_path = Path::new("./out/").join("dispatch.rs");

    let output = format!(
        r#"const QUERY_DISPATCH: &str = "{}";
const QUERY_GATEWAY: &str = "{}";"#,
        dispatch_data, gateway_data
    );

    fs::write(dest_path, output).expect("Unable to write file");
}
