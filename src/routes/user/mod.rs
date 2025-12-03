use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::{
    app_state::{
        AppState,
        link::LinkType::{Download, Upload},
    },
    extractors::user_auth_extractor::UserAuthSessionExtractor,
    localization::app_translator::AppTranslator,
};

pub(crate) mod login;
pub(crate) mod upload;
pub(crate) mod download;

#[derive(Template)]
#[template(path = "invalid-credentials.html")]
struct InvalidCredentialsTemplate<'a> {
    error_msg: &'a str,
}

fn invalid_credentials(t: AppTranslator) -> Html<String> {
    let tmpl = InvalidCredentialsTemplate {
        error_msg: &t.tr("auth_invalidCredentials"),
    };
    Html(tmpl.render().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct FilePath {
    file_url: String,
}

pub async fn load(
    t: AppTranslator,
    _: UserAuthSessionExtractor,
    Path(path): Path<FilePath>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.data.read().await.get_link_type(&path.file_url) {
        Some(link_type) => match link_type {
            Download => download_page(t, &path.file_url),
            Upload => upload_page(t, &path.file_url),
        },
        None => invalid_credentials(t),
    }
}

#[derive(Template)]
#[template(path = "user/download.html")]
struct DownloadPageTemplate<'a> {
    t: AppTranslator,
    url: &'a str
}

fn download_page(t: AppTranslator, url: &str) -> Html<String> {
    let tmpl = DownloadPageTemplate { t, url };
    Html(tmpl.render().unwrap())
}

#[derive(Template)]
#[template(path = "user/upload.html")]
struct UploadPageTemplate<'a> {
    t: AppTranslator,
    url: &'a str,
}

fn upload_page(t: AppTranslator, url: &str) -> Html<String> {
    let tmpl = UploadPageTemplate { t, url };
    Html(tmpl.render().unwrap())
}
