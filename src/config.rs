use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub rpc_url: String,
    pub keypair_path: String,
    pub strategy: StrategyConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrategyConfig {
    pub check_interval_secs: u64,
    pub target_program: String,
    pub transfer_lamports: u64,
    pub max_retries: u8,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let path = Path::new("config/default.toml");
        let raw = fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
        let config = toml::from_str(&raw).context("failed to parse config/default.toml")?;
        Ok(config)
    }
}
