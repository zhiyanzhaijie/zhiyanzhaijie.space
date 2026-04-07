use crate::root::{AppLocale, AppTheme};

fn set_document_attribute(name: &str, value: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(html) = document.document_element() else {
        return;
    };

    if let Err(e) = html.set_attribute(name, value) {
        log::warn!("Failed to set document attribute {name}={value}: {:?}", e);
    }
}

pub fn sync_theme_to_document(theme: AppTheme) {
    set_document_attribute("class", theme.as_str());
}

pub fn sync_locale_to_document(locale: AppLocale) {
    set_document_attribute("lang", locale.as_str());
}
