use crate::models::{GenericSdkRsp, OnMdkGetAgreementDataRsp};
use axum::Json;

pub async fn on_mdk_get_agreement() -> Json<GenericSdkRsp<OnMdkGetAgreementDataRsp>> {
    Json(GenericSdkRsp::<OnMdkGetAgreementDataRsp>::default())
}
