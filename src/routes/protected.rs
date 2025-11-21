use crate::extractors::admin_auth_extractor::AdminAuthSessionExtractor;

pub async fn protected(_: AdminAuthSessionExtractor) {}
