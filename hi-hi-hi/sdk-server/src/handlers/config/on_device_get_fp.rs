use crate::models::{GenericSdkRsp, OnDeviceGetFpDataRsp, OnDeviceGetFpReq};
use axum::Json;

pub async fn on_device_get_fp(
    req: Json<OnDeviceGetFpReq>,
) -> Json<GenericSdkRsp<OnDeviceGetFpDataRsp>> {
    Json(GenericSdkRsp::<OnDeviceGetFpDataRsp> {
        data: OnDeviceGetFpDataRsp {
            code: 200,
            device_fp: req.device_fp.clone(),
            msg: String::from("ok"),
        },
        ..Default::default()
    })
}
