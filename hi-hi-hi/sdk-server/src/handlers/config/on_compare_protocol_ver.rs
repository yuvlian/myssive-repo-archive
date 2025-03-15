use crate::models::{
    GenericSdkRsp, OnCompareProtocolVerDataRsp, OnCompareProtocolVerReq, ProtocolData,
};
use axum::Json;

pub async fn on_compare_protocol_ver(
    req: Json<OnCompareProtocolVerReq>,
) -> Json<GenericSdkRsp<OnCompareProtocolVerDataRsp>> {
    Json(GenericSdkRsp::<OnCompareProtocolVerDataRsp> {
        data: OnCompareProtocolVerDataRsp {
            modified: true,
            protocol: ProtocolData {
                app_id: req.app_id.clone(),
                language: req.language.clone(),
                minimum: 3,
                create_time: String::from("0"),
                ..Default::default()
            },
        },
        ..Default::default()
    })
}
