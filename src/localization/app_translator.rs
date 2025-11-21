use std::{borrow::Cow, collections::HashMap};

use fluent_bundle::FluentValue;
use fluent_templates::{LanguageIdentifier, Loader};

use crate::localization::LOCALES;

#[derive(Debug, Clone)]
pub struct AppTranslator {
    pub lang: LanguageIdentifier,
}

impl AppTranslator {
    pub fn new(lang: LanguageIdentifier) -> Self {
        Self { lang }
    }

    pub fn tr(&self, key: &str) -> String {
        LOCALES.lookup(&self.lang, key)
    }

    pub fn tr_with(&self, key: &str, args: &HashMap<Cow<'static, str>, FluentValue>) -> String {
        LOCALES.lookup_with_args(&self.lang, key, args)
    }
}
