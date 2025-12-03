use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app_state::link::link_config::LinkConfig;

pub(crate) mod link_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Link {
    Download(DownloadLink),
    Upload(UploadLink),
}

pub enum LinkType {
    Download,
    Upload,
}

impl Link {
    fn config(&self) -> &LinkConfig {
        match self {
            Link::Download(download_link) => &download_link.config,
            Link::Upload(upload_link) => &upload_link.config,
        }
    }

    pub fn try_access(&self, psw: &str) -> Result<(), ()> {
        self.config().try_access(psw)
    }

    pub fn get_type(&self) -> LinkType {
        match self {
            Link::Download(_) => LinkType::Download,
            Link::Upload(_) => LinkType::Upload,
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.config().file_name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLink {
    description: Option<String>,
    limited_uses: Option<usize>,
    config: LinkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadLink {
    file_lifetime: Option<DateTime<Utc>>,
    config: LinkConfig,
}

impl Link {
    pub fn new_upload(file_lifetime: Option<DateTime<Utc>>, config: LinkConfig) -> Self {
        Link::Upload(UploadLink {
            file_lifetime,
            config,
        })
    }

    pub fn new_download(
        description: Option<String>,
        limited_uses: Option<usize>,
        config: LinkConfig,
    ) -> Self {
        Link::Download(DownloadLink {
            description,
            limited_uses,
            config,
        })
    }
}
