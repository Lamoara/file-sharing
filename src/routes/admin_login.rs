use std::sync::Arc;

use askama::Template;
use axum::{
    Form,
    extract::State,
    http::{HeaderMap, HeaderValue},
    response::{Html, IntoResponse},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;

use crate::{app_state::AppState, localization::app_translator::AppTranslator};

#[derive(Template)]
#[template(path = "login.html")]
struct AdminLoginTemplate {
    title: &'static str,
}

pub async fn login_page(translator: AppTranslator) -> Html<String> {
    print!("{translator:?}");
    let page = AdminLoginTemplate { title: "Login" };
    Html(page.render().unwrap())
}

#[derive(Template)]
#[template(path = "invalid-credentials.html")]
struct InvalidCredentialsTemplate<'a> {
    error_msg: &'a str,
}

fn invalidCredentials(t: AppTranslator) -> Html<String> {
    let tmpl = InvalidCredentialsTemplate {
        error_msg: &t.tr("auth_invalidCredentials"),
    };
    Html(tmpl.render().unwrap())
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn login_form(
    jar: CookieJar,
    translator: AppTranslator,
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    match state.try_login(&form.username, &form.password).await {
        Ok(uuid) => {
            let cookie = Cookie::build(("session", uuid.to_string()))
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Lax)
                .build();

            let jar = jar.add(cookie);

            let mut headers = HeaderMap::new();
            headers.insert("HX-Redirect", HeaderValue::from_static("admin/dashboard"));

            (headers, jar).into_response()
        }
        Err(_) => invalidCredentials(translator).into_response(),
    }
}
