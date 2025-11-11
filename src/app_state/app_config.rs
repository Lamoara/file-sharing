use std::fs;

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::hasher::hash;

static APP_CONFIG_ROUTE: &str = "app_config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    hashed_admin_username: String,
    hashed_password: String,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<AppConfig> {
        match fs::read_to_string(APP_CONFIG_ROUTE) {
            Ok(data) => {
                info!("Loading app config from file");
                Ok(serde_json::from_str::<AppConfig>(&data)?)
            }
            Err(_) => {
                info!("{APP_CONFIG_ROUTE} not found, creating default AppConfig");
                let app_config = AppConfig::default();
                info!("Saving new AppConfig");
                app_config.save()?;
                Ok(app_config)
            }
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let app_config_json = serde_json::to_string_pretty(&self)?;
        debug!("Saving AppConfig");
        fs::write(APP_CONFIG_ROUTE, app_config_json)?;
        Ok(())
    }
    pub fn change_username() {}
    pub fn change_password() {}
    pub fn validate_credentials() {}
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hashed_admin_username: hash("admin").unwrap(),
            hashed_password: hash("admin").unwrap(),
        }
    }
}
