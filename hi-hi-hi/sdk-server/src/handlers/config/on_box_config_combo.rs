use crate::models::{
    BoxConfigVals, GenericSdkRsp, KibanaPcConfig, NetworkReportConfig, OnBoxConfigComboDataRsp,
};
use axum::Json;

pub async fn on_box_config_combo() -> Json<GenericSdkRsp<OnBoxConfigComboDataRsp>> {
    Json(GenericSdkRsp::<OnBoxConfigComboDataRsp> {
        data: OnBoxConfigComboDataRsp {
            vals: BoxConfigVals {
                network_report_config: NetworkReportConfig {
                    status_codes: vec![206],
                    url_paths: vec![String::from("dataUpload"), String::from("red_dot")],
                    ..Default::default()
                },
                kibana_pc_config: KibanaPcConfig {
                    enable: 1,
                    level: String::from("Debug"),
                    modules: vec![String::from("download")],
                },
                ..Default::default()
            },
        },
        ..Default::default()
    })
}
