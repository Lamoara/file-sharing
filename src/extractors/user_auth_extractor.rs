use std::sync::Arc;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::Response,
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::app_state::{AppState, link_config::LinkRoute};

pub struct UserAuthSessionExtractor(LinkRoute);

impl FromRequestParts<Arc<AppState>> for UserAuthSessionExtractor {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| redirect_to_login())?;

        let session_cookie = cookies.get("session").ok_or_else(redirect_to_login)?;

        let session_id =
            Uuid::parse_str(session_cookie.value()).map_err(|_| redirect_to_login())?;

        let link = state
            .sessions
            .read()
            .await
            .verify_user(&session_id)
            .ok_or_else(redirect_to_login)?;

        Ok(UserAuthSessionExtractor(link))
    }
}

fn redirect_to_login() -> Response {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/admin")
        .body("".into())
        .unwrap()
}
