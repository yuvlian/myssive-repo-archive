use crate::models::{GenericSdkRsp, OnRiskyApiCheckDataRsp};
use axum::Json;

pub async fn on_risky_api_check() -> Json<GenericSdkRsp<OnRiskyApiCheckDataRsp>> {
    Json(GenericSdkRsp::<OnRiskyApiCheckDataRsp> {
        message: String::new(),
        data: OnRiskyApiCheckDataRsp {
            action: String::from("ACTION_NONE"),
            ..Default::default()
        },
        ..Default::default()
    })
}
