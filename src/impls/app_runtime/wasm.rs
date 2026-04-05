use crate::root::{AppLocale, AppTheme};

const THEME_STORAGE_KEY: &str = "app_theme";
const LOCALE_STORAGE_KEY: &str = "app_locale";

fn local_storage() -> Option<web_sys::Storage> {
    let Some(window) = web_sys::window() else {
        log::warn!("Window object not found");
        return None;
    };

    match window.local_storage() {
        Ok(Some(storage)) => Some(storage),
        Ok(None) => {
            log::warn!("Local storage is not available");
            None
        }
        Err(e) => {
            log::warn!("Failed to access local storage: {:?}", e);
            None
        }
    }
}

pub fn load_theme() -> Option<AppTheme> {
    let storage = local_storage()?;
    match storage.get_item(THEME_STORAGE_KEY) {
        Ok(Some(saved_theme)) => {
            let theme = match saved_theme.as_str() {
                "dark" => AppTheme::Dark,
                _ => AppTheme::Light,
            };
            log::info!("Loaded theme from local storage: {}", saved_theme);
            Some(theme)
        }
        Ok(None) => {
            log::info!("No saved theme found, using default");
            None
        }
        Err(e) => {
            log::warn!("Failed to load theme from local storage: {:?}", e);
            None
        }
    }
}

pub fn load_locale_or_default() -> AppLocale {
    let Some(storage) = local_storage() else {
        return AppLocale::default();
    };

    match storage.get_item(LOCALE_STORAGE_KEY) {
        Ok(Some(saved_locale)) => {
            let locale = match saved_locale.as_str() {
                "en" => AppLocale::EN,
                _ => AppLocale::CN,
            };
            log::info!("Loaded locale from local storage: {}", saved_locale);
            locale
        }
        Ok(None) => {
            let locale = AppLocale::default();
            log::info!("No saved locale found, using default (CN)");
            if let Err(e) = storage.set_item(LOCALE_STORAGE_KEY, locale.as_str()) {
                log::error!("Failed to save default locale to local storage: {:?}", e);
            }
            locale
        }
        Err(e) => {
            log::warn!("Failed to load locale from local storage: {:?}", e);
            AppLocale::default()
        }
    }
}

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
