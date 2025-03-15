use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct TokenData {
    #[serde(rename = "channelToken")]
    pub channel_token: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct CodeData {
    pub code: String,
}
