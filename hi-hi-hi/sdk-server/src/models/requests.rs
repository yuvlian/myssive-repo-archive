use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct OnComboGranterLoginReq {
    // pub app_id: u8,
    // pub channel_id: u8,
    pub data: String,
    // pub device: String,
    // pub sign: String,
}

#[derive(Deserialize)]
pub struct ComboLoginData {
    pub uid: String,
    pub guest: bool,
    pub token: String,
}

impl Default for ComboLoginData {
    fn default() -> Self {
        Self {
            uid: String::from("1"),
            guest: false,
            token: String::from("YLN"),
        }
    }
}

impl OnComboGranterLoginReq {
    pub fn extract_data(&self) -> ComboLoginData {
        serde_json::from_str(&self.data).unwrap_or_default()
    }
}

#[derive(Deserialize, Clone)]
pub struct OnMdkLoadConfigQuery {
    pub game_key: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct OnDeviceGetFpReq {
    // pub app_name: String,
    pub device_fp: String,
    // pub device_id: String,
    // pub ext_fields: String,
    // pub platform: String,
    // pub seed_id: String,
    // pub seed_time: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OnCompareProtocolVerReq {
    pub app_id: String,
    pub language: String,
}

// #[derive(Deserialize, Clone)]
// pub struct VersionQuery {
//     pub version: String,
// }
