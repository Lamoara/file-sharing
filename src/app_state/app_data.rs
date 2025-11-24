use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use uuid::Uuid;

use crate::app_state::link::Link;

static APP_DATA_ROUTE: &str = "app_data.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    links: HashMap<String, Arc<Link>>,
    files: HashSet<String>,
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
                    files: Default::default(),
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

    pub fn links(&self) -> Vec<Arc<Link>> {
        self.links.values().cloned().collect()
    }

    pub fn add_link(&mut self, link_route: Option<String>, link: Link) -> Result<(), ()> {
        let route = link_route.unwrap_or_else(|| Uuid::new_v4().to_string());
        self.links.insert(route, Arc::new(link)); //TODO! Make proper errors
        self.save().unwrap(); //TODO! Make proper errors
        Ok(())
    }
    pub fn remove_link() {}
    pub fn remove_links() {}
    pub fn update_link() {}
    pub fn clean() {}
}
