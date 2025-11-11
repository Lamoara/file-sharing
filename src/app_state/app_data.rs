use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::app_state::link_config::LinkConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    links: HashMap<String, LinkConfig>,
}

impl AppData {
    pub fn load() -> AppData {
        AppData {
            links: Default::default(),
        }
    }
    pub fn save() {}
    pub fn add_link() {}
    pub fn remove_link() {}
    pub fn remove_links() {}
    pub fn update_link() {}
    pub fn clean() {}
}
