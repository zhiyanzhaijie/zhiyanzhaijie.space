use dioxus::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

use crate::IO::user::SessionPreferenceDto;

pub type PreferenceContext = Signal<SessionPreferenceDto>;

#[component]
pub fn PreferenceProvider(initial: SessionPreferenceDto, children: Element) -> Element {
    let preference: PreferenceContext = use_signal(|| initial.clone());
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
