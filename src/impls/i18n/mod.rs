use dioxus_i18n::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

pub const DEFAULT_LANGUAGE: LanguageIdentifier = langid!("zh-CN");
pub const EN_US_LANGUAGE: LanguageIdentifier = langid!("en-US");

pub fn build_i18n_config() -> I18nConfig {
    I18nConfig::new(DEFAULT_LANGUAGE)
        .with_locale((DEFAULT_LANGUAGE, include_str!("./locales/zh-CN.ftl")))
        .with_locale((EN_US_LANGUAGE, include_str!("./locales/en-US.ftl")))
        .with_fallback(DEFAULT_LANGUAGE)
}
