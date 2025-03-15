use crate::models::{AccountData, GenericSdkRsp, OnMdkShieldApiDataRsp};
use axum::Json;

pub async fn on_mdk_shield_api() -> Json<GenericSdkRsp<OnMdkShieldApiDataRsp>> {
    Json(GenericSdkRsp::<OnMdkShieldApiDataRsp> {
        data: OnMdkShieldApiDataRsp {
            account: AccountData {
                uid: String::from("1"),
                name: String::from("yulian"),
                is_email_verify: String::from("0"),
                token: String::from("YLN"),
                country: String::from("SG"),
                area_code: String::from("**"),
                ..Default::default()
            },
            realname_operation: String::from("None"),
            ..Default::default()
        },
        ..Default::default()
    })
}
