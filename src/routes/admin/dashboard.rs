use std::{str::FromStr, sync::Arc};

use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::{
    app_state::{
        AppState,
        link::{Link, UploadLink, link_config::LinkConfig},
    },
    extractors::{
        admin_auth_extractor::AdminAuthSessionExtractor,
        user_auth_extractor::UserAuthSessionExtractor,
    },
};

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
struct DashboardTemplate;

fn dashboard_html() -> Html<String> {
    Html(DashboardTemplate.render().unwrap())
}

pub async fn dashboard(_: AdminAuthSessionExtractor) -> impl IntoResponse {
    dashboard_html()
}

#[derive(Default, Deserialize)]
pub struct UploadLinkForm {
    name: Option<String>,
    file_lifetime: Option<String>,
    password: Option<String>,
    expiration_date: Option<String>,
}

pub async fn create_upload_link(
    _: AdminAuthSessionExtractor,
    State(state): State<Arc<AppState>>,
    Form(form): Form<UploadLinkForm>,
) {
    let name = form.name.filter(|s| !s.is_empty());
    let file_lifetime = form.file_lifetime.filter(|s| !s.is_empty()).map(|naive| {
        NaiveDateTime::parse_from_str(&naive, "%Y-%m-%dT%H:%M")
            .unwrap()
            .and_utc()
    });
    let password = form.password.filter(|s| !s.is_empty());
    let expiration_date = form.expiration_date.filter(|s| !s.is_empty()).map(|naive| {
        NaiveDateTime::parse_from_str(&naive, "%Y-%m-%dT%H:%M")
            .unwrap()
            .and_utc()
    });

    let link_config = LinkConfig::new(password, expiration_date);
    let link = Link::new_upload(file_lifetime, link_config);

    state.create_link(name, link).await.unwrap();
}

#[derive(Default, Deserialize)]
pub struct DownloadLinkForm {
    name: Option<String>,
    file: String,
    description: Option<String>,
    #[serde(deserialize_with = "parse_usize_opt::deserialize")]
    limited_uses: Option<usize>,
    password: Option<String>,
    expiration_date: Option<String>,
}

pub async fn create_download_link(
    _: AdminAuthSessionExtractor,
    State(state): State<Arc<AppState>>,
    Form(form): Form<DownloadLinkForm>,
) {
    let name = form.name.filter(|s| !s.is_empty());
    let file = form.file;
    let description = form.description.filter(|s| !s.is_empty());
    let limited_uses = form.limited_uses;
    let password = form.password.filter(|s| !s.is_empty());
    let expiration_date = form.expiration_date.filter(|s| !s.is_empty()).map(|naive| {
        NaiveDateTime::parse_from_str(&naive, "%Y-%m-%dT%H:%M")
            .unwrap()
            .and_utc()
    });

    let link_config = LinkConfig::new(password, expiration_date);
    let link = Link::new_download(file, description, limited_uses, link_config);

    state.create_link(name, link).await.unwrap();
}

mod filters {
    pub fn display_some<T>(value: &Option<T>, _: &dyn askama::Values) -> askama::Result<String>
    where
        T: std::fmt::Display,
    {
        Ok(match value {
            Some(value) => value.to_string(),
            None => String::new(),
        })
    }
}

mod parse_usize_opt {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            None => Ok(None),
            Some(s) if s.trim().is_empty() => Ok(None),
            Some(s) => s
                .parse::<usize>()
                .map(Some)
                .map_err(serde::de::Error::custom),
        }
    }
}
