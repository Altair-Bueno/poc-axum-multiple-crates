use std::net::SocketAddr;

use figment::providers::Env;
use figment::providers::Format;
use figment::providers::Toml;
use figment::Figment;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostConfig {
    pub collection: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mongo: String,
    pub listen: SocketAddr,
    pub database: String,
    pub post: PostConfig,
}

pub fn config() -> Result<Config, figment::Error> {
    Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("MYAPP_"))
        .extract()
}
