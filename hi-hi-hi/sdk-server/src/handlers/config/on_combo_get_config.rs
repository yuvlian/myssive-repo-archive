use crate::models::{GenericSdkRsp, OnComboGetConfigDataRsp};
use axum::Json;

pub async fn on_combo_get_config() -> Json<GenericSdkRsp<OnComboGetConfigDataRsp>> {
    Json(GenericSdkRsp::<OnComboGetConfigDataRsp> {
        data: OnComboGetConfigDataRsp {
            protocol: true,
            log_level: String::from("DEBUG"),
            announce_url: String::from("http://127.0.0.1:21000/announcement/"),
            push_alias_type: 2,
            app_name: String::from("崩坏3-东南亚"),
            ..Default::default()
        },
        ..Default::default()
    })
}
