use dioxus::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

use crate::IO::user::SessionPreferenceDto;
#[derive(Clone, Store)]
pub struct PreferenceStore {
    pub locale: Option<String>,
    pub theme: Option<String>,
}

impl From<SessionPreferenceDto> for PreferenceStore {
    fn from(value: SessionPreferenceDto) -> Self {
        Self {
            locale: value.locale,
            theme: value.theme,
        }
    }
}

pub type PreferenceContext = Store<PreferenceStore>;

#[component]
pub fn PreferenceProvider(initial: SessionPreferenceDto, children: Element) -> Element {
    let preference: PreferenceContext = use_store(|| initial.clone().into());
    use_context_provider(|| preference);

    children
}

pub fn resolve_locale(locale: Option<&str>) -> &'static str {
    match locale {
        Some("en") => "en",
        _ => "cn",
    }
}

pub fn resolve_theme(theme: Option<&str>) -> &'static str {
    match theme {
        Some("dark") => "dark",
        _ => "light",
    }
}

pub fn locale_to_langid(locale: Option<&str>) -> LanguageIdentifier {
    match resolve_locale(locale) {
        "en" => langid!("en-US"),
        _ => langid!("zh-CN"),
    }
}
