use crate::models::{GenericSdkRsp, GuestData, OnComboGranterLoginDataRsp, OnComboGranterLoginReq};
use axum::Json;

pub async fn on_combo_granter_login(
    req: Json<OnComboGranterLoginReq>,
) -> Json<GenericSdkRsp<OnComboGranterLoginDataRsp>> {
    let data = req.extract_data();

    Json(GenericSdkRsp::<OnComboGranterLoginDataRsp> {
        data: OnComboGranterLoginDataRsp {
            combo_id: String::from("0"),
            account_type: 1,
            open_id: data.uid,
            combo_token: data.token,
            data: GuestData { guest: data.guest },
            ..Default::default()
        },
        ..Default::default()
    })
}
