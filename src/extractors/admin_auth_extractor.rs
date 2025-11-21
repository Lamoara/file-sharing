use std::sync::Arc;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::Response,
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::app_state::AppState;

pub struct AdminAuthSessionExtractor;

impl FromRequestParts<Arc<AppState>> for AdminAuthSessionExtractor {
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

        state
            .sessions
            .read()
            .await
            .verify_admin(&session_id)
            .ok_or_else(redirect_to_login)?;

        Ok(AdminAuthSessionExtractor)
    }
}

fn redirect_to_login() -> Response {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/admin")
        .body("".into())
        .unwrap()
}
