use std::{borrow::Cow, collections::HashMap};

use axum::response::IntoResponse;

use crate::{
    extractors::admin_auth_extractor::AdminAuthSessionExtractor,
    localization::{app_translator::AppTranslator, arg_map::ArgMapBuilder},
};

pub async fn protected(
    _: AdminAuthSessionExtractor,
    translator: AppTranslator,
) -> impl IntoResponse {
    let args = ArgMapBuilder::default()
        .arg_str("name", "pepe")
        .build();
    translator.tr_with("admin_hello", &args)
}
