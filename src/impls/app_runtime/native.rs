use crate::root::{AppLocale, AppTheme};

pub fn load_theme() -> Option<AppTheme> {
    None
}

pub fn load_locale_or_default() -> AppLocale {
    AppLocale::default()
}

pub fn sync_theme_to_document(_theme: AppTheme) {}

pub fn sync_locale_to_document(_locale: AppLocale) {}
