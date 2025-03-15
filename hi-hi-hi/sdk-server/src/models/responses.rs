use serde::Serialize;

#[derive(Serialize)]
pub struct GenericSdkRsp<T> {
    pub retcode: i8,
    pub message: String,
    pub data: T,
}

impl<T: Default> Default for GenericSdkRsp<T> {
    fn default() -> Self {
        Self {
            retcode: 0,
            message: String::from("OK"),
            data: T::default(),
        }
    }
}

#[derive(Serialize, Default)]
pub struct OnRiskyApiCheckDataRsp {
    pub id: String,
    pub action: String,
    pub geetest: Option<serde_json::Value>,
}

#[derive(Serialize, Default)]
pub struct OnComboGranterLoginDataRsp {
    pub combo_id: String,
    pub open_id: String,
    pub combo_token: String,
    pub data: GuestData,
    pub heartbeat: bool,
    pub account_type: u8,
}

#[derive(Serialize, Default)]
pub struct GuestData {
    pub guest: bool,
}

#[derive(Serialize, Default)]
pub struct OnMdkShieldApiDataRsp {
    pub account: AccountData,
    pub device_grant_required: bool,
    pub safe_mobile_required: bool,
    pub realperson_required: bool,
    pub reactivate_required: bool,
    pub realname_operation: String,
}

#[derive(Serialize, Default)]
pub struct AccountData {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub is_email_verify: String,
    pub realname: String,
    pub identity_card: String,
    pub token: String,
    pub safe_mobile: String,
    pub facebook_name: String,
    pub google_name: String,
    pub twitter_name: String,
    pub game_center_name: String,
    pub apple_name: String,
    pub sony_name: String,
    pub tap_name: String,
    pub country: String,
    pub reactivate_ticket: String,
    pub area_code: String,
    pub device_grant_ticket: String,
    pub steam_name: String,
    pub unmasked_email: String,
    pub unmasked_email_type: u8,
}

#[derive(Serialize, Default)]
pub struct OnMdkGetAgreementDataRsp {
    pub marketing_agreements: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct OnListExperimentRsp {
    pub retcode: i8,
    pub success: bool,
    pub message: String,
    pub data: Vec<serde_json::Value>,
}

#[derive(Serialize, Default)]
pub struct OnListDevicesDataRsp {
    pub devices: Vec<serde_json::Value>,
    pub latest_id: String,
}

#[derive(Serialize, Default)]
pub struct OnComboGetConfigDataRsp {
    pub protocol: bool,
    pub qr_enabled: bool,
    pub log_level: String,
    pub announce_url: String,
    pub push_alias_type: u8,
    pub disable_ysdk_guard: bool,
    pub enable_announce_pic_popup: bool,
    pub app_name: String,
    pub qr_enabled_apps: QrEnabledApps,
    pub qr_app_icons: QrAppIcons,
    pub qr_cloud_display_name: String,
}

#[derive(Serialize, Default)]
pub struct QrEnabledApps {
    pub bbs: bool,
    pub cloud: bool,
}

#[derive(Serialize, Default)]
pub struct QrAppIcons {
    pub app: String,
    pub bbs: String,
    pub cloud: String,
}

#[derive(Serialize, Default)]
pub struct OnGetWeatherDataRsp {
    pub timezone: u8,
    pub hourly: Vec<HourlyWeather>,
}

#[derive(Serialize, Default)]
pub struct HourlyWeather {
    pub condition: u8,
    pub date: String,
    pub hour: u8,
    pub temp: i8,
}

#[derive(Serialize, Default)]
pub struct OnMdkLoadConfigDataRsp {
    pub id: u8,
    pub game_key: String,
    pub client: String,
    pub identity: String,
    pub guest: bool,
    pub ignore_versions: String,
    pub scene: String,
    pub name: String,
    pub disable_regist: bool,
    pub enable_email_captcha: bool,
    pub thirdparty: Vec<String>,
    pub disable_mmt: bool,
    pub server_guest: bool,
    pub thirdparty_ignore: serde_json::Value,
    pub enable_ps_bind_account: bool,
    pub thirdparty_login_configs: serde_json::Value,
    pub initialize_firebase: bool,
}

#[derive(Serialize, Default)]
pub struct OnBoxConfigComboDataRsp {
    pub vals: BoxConfigVals,
}

#[derive(Serialize, Default)]
pub struct BoxConfigVals {
    pub list_price_tierv2_enable: String,
    pub network_report_config: NetworkReportConfig,
    pub kibana_pc_config: KibanaPcConfig,
}

#[derive(Serialize, Default)]
pub struct NetworkReportConfig {
    pub enable: u8,
    pub status_codes: Vec<u16>,
    pub url_paths: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct KibanaPcConfig {
    pub enable: u8,
    pub level: String,
    pub modules: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct OnGetExtListDataRsp {
    pub code: u16,
    pub msg: String,
    pub ext_list: Vec<String>,
    pub pkg_list: Vec<String>,
    pub pkg_str: String,
}

#[derive(Serialize, Default)]
pub struct OnDeviceGetFpDataRsp {
    pub code: u16,
    pub device_fp: String,
    pub msg: String,
}

pub type Report = String;

#[derive(Serialize, Default)]
pub struct OnAdminMi18nRsp {
    pub version: u16,
}

#[derive(Serialize, Default)]
pub struct OnDataUploadRsp {
    pub code: u16,
}

#[derive(Serialize, Default)]
pub struct OnCompareProtocolVerDataRsp {
    pub modified: bool,
    pub protocol: ProtocolData,
}

#[derive(Serialize, Default)]
pub struct ProtocolData {
    pub id: u32,
    pub app_id: String,
    pub language: String,
    pub user_proto: String,
    pub priv_proto: String,
    pub major: u8,
    pub minimum: u8,
    pub create_time: String,
    pub teenager_proto: String,
    pub third_proto: String,
}

#[derive(Serialize, Default)]
pub struct OnQueryDispatchRsp {
    pub retcode: i8,
    pub region_list: Vec<RegionInfo>,
}

#[derive(Serialize, Default)]
pub struct RegionInfo {
    pub retcode: i8,
    pub dispatch_url: String,
    pub name: String,
    pub title: String,
}

#[derive(Serialize, Default)]
pub struct OnQueryGatewayRsp {
    pub account_url: String,
    pub account_url_backup: String,
    pub asset_bundle_url_list: Vec<String>,
    pub ex_resource_url_list: Vec<String>,
    pub ext: serde_json::Value,
    pub gameserver: GateAddr,
    pub gateway: GateAddr,
    pub is_data_ready: bool,
    pub get_hotpatch_manifest: serde_json::Value,
    pub msg: String,
    pub oaserver_url: String,
    pub region_name: String,
    pub retcode: i8,
    pub server_cur_time: u64,
    pub server_cur_timezone: u8,
    pub server_ext: GatewayExt,
}

#[derive(Serialize, Default)]
pub struct GateAddr {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Default)]
pub struct GatewayExt {
    pub cdkey_url: String,
    pub is_official: String,
    pub mihoyo_sdk_env: String,
    pub use_account_web_url: String,
}
