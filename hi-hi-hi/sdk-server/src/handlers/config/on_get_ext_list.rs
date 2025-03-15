use crate::models::{GenericSdkRsp, OnGetExtListDataRsp};
use axum::Json;

pub async fn on_get_ext_list() -> Json<GenericSdkRsp<OnGetExtListDataRsp>> {
    let ext_list = vec![
        String::from("cpuName"),
        String::from("deviceModel"),
        String::from("deviceName"),
        String::from("deviceType"),
        String::from("deviceUID"),
        String::from("gpuID"),
        String::from("gpuName"),
        String::from("gpuAPI"),
        String::from("gpuVendor"),
        String::from("gpuVersion"),
        String::from("gpuMemory"),
        String::from("osVersion"),
        String::from("cpuCores"),
        String::from("cpuFrequency"),
        String::from("gpuVendorID"),
        String::from("isGpuMultiTread"),
        String::from("memorySize"),
        String::from("screenSize"),
        String::from("engineName"),
        String::from("addressMAC"),
    ];

    Json(GenericSdkRsp::<OnGetExtListDataRsp> {
        data: OnGetExtListDataRsp {
            code: 200,
            pkg_str: String::from("/vK5WTh5SS3SAj8Zm0qPWg=="),
            ext_list,
            ..Default::default()
        },
        ..Default::default()
    })
}
