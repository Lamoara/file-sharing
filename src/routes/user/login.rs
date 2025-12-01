use std::sync::Arc;

use askama::Template;
use axum::{
    Form, extract::{Path, Query, State}, http::{HeaderMap, HeaderValue, StatusCode}, response::{Html, IntoResponse, Response}
};
use serde::Deserialize;

use crate::{app_state::AppState, localization::app_translator::AppTranslator};

use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

#[derive(Debug, Deserialize)]
pub struct LoginPath {
    file_url: String,
}

#[derive(Template)]
#[template(path = "user/login.html")]
struct UserLoginTemplate<'a> {
    t: AppTranslator,
    url: &'a str
}

pub async fn login_page(t: AppTranslator, Path(path): Path<LoginPath>,) -> Html<String> {
    let page = UserLoginTemplate { t, url: &path.file_url };
    Html(page.render().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    password: String,
}

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

pub async fn login_form(
    jar: CookieJar,
    t: AppTranslator,
    State(state): State<Arc<AppState>>,
    Path(path): Path<LoginPath>,
    Form(params): Form<LoginInfo>,
) -> Response {
    let url = path.file_url;
    let psw = params.password;

    match state.data.read().await.try_access(&url, &psw) {
        Err(_) => invalid_credentials(t).into_response(),
        Ok(_) => {
            let id = state.sessions.write().await.login_user(url.clone());

            let cookie = Cookie::build(("session", id.to_string()))
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Lax)
                .build();

            let jar = jar.add(cookie);

            let mut headers = HeaderMap::new();

            headers.insert("HX-Redirect", HeaderValue::from_static("../"));

            (headers, jar).into_response()
        }
    }
}
