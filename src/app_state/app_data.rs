use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use uuid::Uuid;

use crate::app_state::link::{Link, LinkType};

static APP_DATA_ROUTE: &str = "app_data.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    links: HashMap<String, Arc<Link>>,
    files: HashMap<String, String>, //Filename, original name
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
        if link.get_type() == LinkType::Upload {
            self.files.insert(link.get_filename().into(), "".into());
        }
        self.links.insert(route, Arc::new(link)); //TODO! Make proper errors
        self.save().unwrap(); //TODO! Make proper errors
        Ok(())
    }

    pub fn get_link_type(&self, link_route: &str) -> Option<LinkType> {
        match self.links.get(link_route) {
            Some(link) => Some(link.get_type()),
            None => None,
        }
    }

    pub fn get_link_filename(&self, link_route: &str) -> Option<&str> {
        let Some(link) = self.links.get(link_route) else {
            return None;
        };

        Some(link.get_filename())
    }

    pub fn save_file_original_name(&mut self, filename: &str, original_name: &str) {
        self.files.insert(filename.to_string(), original_name.to_string());
        self.save().unwrap()
    }

    pub fn get_original_name(&self, filename: &str) -> Option<&str> {
        self.files.get(filename).map(|x| x.as_str())
    }

    pub fn remove_link() {}
    pub fn remove_links() {}
    pub fn update_link() {}
    pub fn clean() {}

    pub fn try_access(&self, url: &str, psw: &str) -> Result<(), ()> {
        self.links.get(url).ok_or_else(|| ())?.try_access(psw)
    }
}
