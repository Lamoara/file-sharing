use std::{convert::Infallible, str::FromStr, sync::Arc};

use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use fluent_templates::LanguageIdentifier;
use tracing::debug;

use crate::{app_state::AppState, localization::app_translator::AppTranslator};

impl FromRequestParts<Arc<AppState>> for AppTranslator {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state).await.unwrap();

        if let Some(lang) = cookies.get("lang")
            && let Ok(lang) = LanguageIdentifier::from_str(lang.value())
        {
            debug!("Using lang cookie for language");
            return Ok(AppTranslator::new(lang));
        }

        if let Some(lang) = parts.headers.get("Accept-Language")
            && let Ok(lang) = lang.to_str()
            && let Some(lang) = lang.split(";").next()
            && let Some(lang) = lang.split(",").next()
            && let Ok(lang) = LanguageIdentifier::from_str(lang)
        {
            debug!("Using Accept-Language cookie for language");
            return Ok(AppTranslator::new(lang));
        }

        debug!("Using default language, no cookie found");
        Ok(AppTranslator::new(
            LanguageIdentifier::from_str("en").unwrap(),
        ))
    }
}
