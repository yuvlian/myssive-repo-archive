use crate::models::{GenericSdkRsp, OnMdkLoadConfigDataRsp, OnMdkLoadConfigQuery};
use axum::{Json, extract::Query};

pub async fn on_mdk_load_config(
    q: Query<OnMdkLoadConfigQuery>,
) -> Json<GenericSdkRsp<OnMdkLoadConfigDataRsp>> {
    let game_key = q.game_key.clone().unwrap_or_default();

    Json(GenericSdkRsp::<OnMdkLoadConfigDataRsp> {
        data: OnMdkLoadConfigDataRsp {
            id: 16,
            game_key,
            client: String::from("PC"),
            identity: String::from("I_IDENTITY"),
            scene: String::from("S_NORMAL"),
            name: String::from("崩坏3rd-东南亚"),
            thirdparty_ignore: serde_json::json!("{}"),
            thirdparty_login_configs: serde_json::json!("{}"),
            ..Default::default()
        },
        ..Default::default()
    })
}
