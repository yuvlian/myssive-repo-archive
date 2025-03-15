use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub sdk_server_host: String,
    pub sdk_server_port: u16,
    pub game_server_host: String,
    pub game_server_port: u32,
    pub dispatch_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            sdk_server_host: String::from("127.0.0.1"),
            sdk_server_port: 21000,
            game_server_host: String::from("127.0.0.1"),
            game_server_port: 23301,
            dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
        }
    }
}

impl ServerConfig {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path);
        match content {
            Ok(data) => toml::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}
