use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app_state::link::link_config::LinkConfig;

pub(crate) mod link_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Link {
    Download(DownloadLink),
    Upload(UploadLink),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLink {
    file: String,
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
        file: String,
        description: Option<String>,
        limited_uses: Option<usize>,
        config: LinkConfig,
    ) -> Self {
        Link::Download(DownloadLink {
            file,
            description,
            limited_uses,
            config,
        })
    }
}
