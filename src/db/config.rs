use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub connect_string: String,
}

impl DbConfig {
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let text = fs::read_to_string(path)?;
        let cfg: DbConfig = toml::from_str(&text)?;
        Ok(cfg)
    }
}
