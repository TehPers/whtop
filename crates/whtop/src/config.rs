use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub refresh_rate_secs: f32,
    pub address: SocketAddr,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            refresh_rate_secs: 1.0,
            address: ([0, 0, 0, 0], 8080).into(),
        }
    }
}
