use crate::models::{GenericSdkRsp, OnListDevicesDataRsp};
use axum::Json;

pub async fn on_list_devices() -> Json<GenericSdkRsp<OnListDevicesDataRsp>> {
    Json(GenericSdkRsp::<OnListDevicesDataRsp> {
        data: OnListDevicesDataRsp {
            latest_id: String::from("0"),
            ..Default::default()
        },
        ..Default::default()
    })
}
