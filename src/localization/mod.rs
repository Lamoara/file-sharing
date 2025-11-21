use fluent_templates::static_loader;

pub(crate) mod app_translator;
pub(crate) mod arg_map;

static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}
