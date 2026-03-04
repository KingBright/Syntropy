use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub namespace: String,
    pub database_name: String,
}

impl AppConfig {
    /// Loads the configuration merging defaults, Config.toml, and Environment Variables.
    /// ENV vars follow the pattern: SYNTROPY_SERVER__PORT
    pub fn load() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("Config.toml"))
            .merge(Env::prefixed("SYNTROPY_").split("__"))
            .extract()
    }
}
