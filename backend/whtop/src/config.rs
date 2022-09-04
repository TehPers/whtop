use serde::Deserialize;
use std::{
    net::{Ipv6Addr, SocketAddr},
    path::PathBuf,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// The address to listen on.
    pub address: SocketAddr,
    /// The maximum refresh rate for system information.
    pub refresh_rate_secs: f32,
    /// Whether to serve static assets.
    pub serve_static: bool,
    /// The path to the static files to serve.
    pub static_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            refresh_rate_secs: 2.0,
            address: (Ipv6Addr::UNSPECIFIED, 8080).into(),
            serve_static: true,
            static_dir: "dist".into(),
        }
    }
}
