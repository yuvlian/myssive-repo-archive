use ohkami::{Ohkami, Route};

const RISKY_API_CHECK: &str = r#"{"data":{},"message":"OK","retcode":0}"#;
const MDK_SHIELD_API: &str = r#"{"data":{"account":{"area_code":"**","email":"a@a.a","country":"ID","is_email_verify":"1","token":"a","uid":"1"},"device_grant_required":false,"reactivate_required":false,"realperson_required":false,"safe_mobile_required":false},"message":"OK","retcode":0}"#;
const COMBO_GRANTER_LOGIN: &str = r#"{"data":{"account_type":1,"combo_id":"1","combo_token":"a","data":"{\"guest\":false}","heartbeat":false,"open_id":"1"},"message":"OK","retcode":0}"#;
include!("../out/dispatch.rs");

async fn mdk_shield() -> String {
    String::from(MDK_SHIELD_API)
}

async fn login_granter() -> String {
    String::from(COMBO_GRANTER_LOGIN)
}

async fn risky_api_check() -> String {
    String::from(RISKY_API_CHECK)
}

async fn query_dispatch() -> String {
    String::from(QUERY_DISPATCH)
}

async fn query_gateway() -> String {
    String::from(QUERY_GATEWAY)
}

pub fn main() {
    smol::block_on(async {
        Ohkami::new((
            "/query_dispatch".GET(query_dispatch),
            "/query_gateway".GET(query_gateway),
            "/account/risky/api/check".POST(risky_api_check),
            "/hkrpg_global/mdk/shield/api/login".POST(mdk_shield),
            "/hkrpg_global/mdk/shield/api/verify".POST(mdk_shield),
            "/hkrpg_global/combo/granter/login/v2/login".POST(login_granter),
        ))
        .howl("127.0.0.1:21000")
        .await
    })
}
