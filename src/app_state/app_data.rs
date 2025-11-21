use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::app_state::link_config::{LinkConfig, LinkRoute};

static APP_DATA_ROUTE: &str = "app_data.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    links: HashMap<LinkRoute, LinkConfig>,
}

impl AppData {
    pub fn load() -> anyhow::Result<AppData> {
        match fs::read_to_string(APP_DATA_ROUTE) {
            Ok(data) => {
                info!("Loading app data from file");
                Ok(serde_json::from_str::<AppData>(&data)?)
            }
            Err(_) => {
                info!("{APP_DATA_ROUTE} not found, creating default AppData");
                let app_data = AppData {
                    links: Default::default(),
                };
                info!("Saving new AppData");
                app_data.save()?;
                Ok(app_data)
            }
        }
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let app_data_json = serde_json::to_string_pretty(&self)?;
        debug!("Saving new AppData");
        fs::write(APP_DATA_ROUTE, app_data_json)?;
        Ok(())
    }
    pub fn add_link() {}
    pub fn remove_link() {}
    pub fn remove_links() {}
    pub fn update_link() {}
    pub fn clean() {}
}
