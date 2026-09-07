use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration loading failed: {0}")]
    Load(#[from] config::ConfigError),
    #[error("Environment file error: {0}")]
    Dotenv(#[from] dotenvy::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub app: AppSettings,
    pub server: ServerSettings,
    pub db: DatabaseSettings,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let s = config::Config::builder()
            .add_source(config::Environment::with_prefix("PLATO").separator("__"))
            .set_default("app.name", "plato-api")?
            .set_default("app.version", "0.1.0")?.set_default("app.jwt_secret", "supersecret-dev-key-123")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("db.url", "postgres://postgres:postgres@localhost:5432/plato")?
            .set_default("db.max_connections", 10)?
            .set_default("db.connection_timeout", 5)?
            .build()?;

        s.try_deserialize().map_err(ConfigError::from)
    }
}
