use serde::Serialize;

// ------ GENERIC MODELS ------

#[derive(Serialize, Default)]
pub struct GRsp<T> {
    pub code: Option<u16>,
    pub msg: String,
    pub status: Option<u16>,
    pub r#type: Option<String>,
    pub data: Option<T>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericAppV1Config {
    pub agreement_url: AgreementUrl,
    pub app: AppInfo,
    pub customer_service_url: String,
    pub third_party_redirect_url: String,
    pub scan_url: ScanUrl,
    pub login_channels: Vec<serde_json::Value>,
    pub user_center_url: String,
}

#[derive(Serialize, Default)]
pub struct GenericToken {
    pub token: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericMeta {
    pub character_list: Vec<CharacterMeta>,
    pub cbt2: Cbt2Info,
    pub sections: Vec<SectionData>,
    pub download: bool,
}

#[derive(Serialize, Default)]
pub struct GenericAuthGrant {
    pub uid: String,
    pub code: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericUserInfo {
    pub hg_id: String,
    pub email: String,
    pub real_email: String,
    pub is_latest_user_agreement: bool,
    pub nickname: String,
}

#[derive(Serialize, Default)]
pub struct GenericQueryZone {
    pub zone: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct GenericVersion {
    pub version: u16,
}

// ------ GENERIC MODELS NESTS -------

#[derive(Serialize, Default)]
pub struct CharacterMeta {
    pub key: String,
    pub code: String,
    pub name: String,
    pub codename: String,
    pub camp: String,
    pub race: String,
    pub intro: String,
    pub cid: String,
    pub anime: bool,
}

#[derive(Serialize, Default)]
pub struct Cbt2Info {
    pub recruitment: RecruitmentInfo,
}

#[derive(Serialize, Default)]
pub struct RecruitmentInfo {
    pub questionnaire: bool,
    pub inquiry: bool,
    pub activity: bool,
}

#[derive(Serialize, Default)]
pub struct SectionData {
    pub key: String,
    pub data: SectionContent,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SectionContent {
    pub pv: Option<String>,
    pub pv_h5: Option<String>,
    pub pv_list: Option<Vec<PvListItem>>,
}

#[derive(Serialize, Default)]
pub struct PvListItem {
    pub title: String,
    pub embed: String,
    pub cover: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgreementUrl {
    pub register: String,
    pub privacy: String,
    pub unbind: String,
    pub account: String,
    pub game: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub google_android_client_id: String,
    pub google_ios_client_id: String,
    pub enable_auto_login: bool,
    pub enable_payment: bool,
    pub enable_guest_register: bool,
    pub need_show_name: bool,
    pub display_name: AppDisplayName,
    pub unbind_agreement: Vec<serde_json::Value>,
    pub unbind_limited_days: u32,
    pub unbind_cool_down_days: u32,
    pub customer_service_url: String,
    pub enable_unbind_grant: bool,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct AppDisplayName {
    pub en_us: String,
    pub ja_jp: String,
    pub ko_kr: String,
    pub zh_cn: String,
    pub zh_tw: String,
}

#[derive(Serialize, Default)]
pub struct ScanUrl {
    pub login: String,
}

// ------ NON GENERIC MODELS ------

#[derive(Serialize, Default)]
pub struct RemoteServerConfig {
    pub addr: String,
    pub port: u16,
}

#[derive(Serialize, Default)]
pub struct NetworkConfig {
    pub asset: String,
    pub hgage: String,
    pub sdkenv: String,
    pub u8root: String,
    pub appcode: u8,
    pub channel: String,
    pub netlogid: String,
    pub gameclose: bool,
    pub netlogurl: String,
    pub accounturl: String,
    pub launcherurl: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteConfig {
    pub mock_login: bool,
    pub select_srv: bool,
    pub enable_hot_update: bool,
    pub enable_entity_spawn_log: bool,
}

#[derive(Serialize, Default)]
pub struct ProductList {
    #[serde(rename = "productList")]
    pub product_list: Vec<serde_json::Value>,
}

#[derive(Serialize, Default)]
pub struct GetLatestVersion {
    pub action: u8,
    pub version: String,
    pub request_version: String,
    pub pkg: LatestVerPkg,
    pub patch: Option<serde_json::Value>,
}

#[derive(Serialize, Default)]
pub struct LatestVerPkg {
    pub packs: Vec<serde_json::Value>,
    pub total_size: String,
    pub file_path: String,
    pub url: String,
    pub md5: String,
    pub package_size: String,
    pub file_id: String,
    pub sub_channel: String,
}
