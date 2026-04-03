pub mod layouts;
mod routes;

use crate::components::animated_bird::AnimatedBird;
use dioxus::document::{Link, Stylesheet};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
pub use routes::Route;
use unic_langid::{langid, LanguageIdentifier};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppTheme {
    Light,
    Dark,
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::Light
    }
}

impl AppTheme {
    pub fn as_str(&self) -> &'static str {
        match *self {
            AppTheme::Light => "light",
            AppTheme::Dark => "dark",
        }
    }
}

pub static ACTIVE_THEME: GlobalSignal<AppTheme> = Signal::global(AppTheme::default);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppLocale {
    CN,
    EN,
}

impl Default for AppLocale {
    fn default() -> Self {
        AppLocale::CN
    }
}

impl AppLocale {
    pub fn as_str(&self) -> &'static str {
        match *self {
            AppLocale::CN => "cn",
            AppLocale::EN => "en",
        }
    }

    pub fn to_langid(&self) -> LanguageIdentifier {
        match *self {
            AppLocale::CN => langid!("zh-CN"),
            AppLocale::EN => langid!("en-US"),
        }
    }
}

pub static ACTIVE_LOCALE: GlobalSignal<AppLocale> = Signal::global(AppLocale::default);
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TW_STYLES: Asset = asset!("/assets/tailwind.css");
const NOISE_IMAGE: Asset = asset!("/assets/noise.png");

#[allow(non_snake_case)]
pub fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("zh-CN"))
            .with_locale((langid!("zh-CN"), include_str!("../i18n/zh-CN/index.ftl")))
            .with_locale((langid!("en-US"), include_str!("../i18n/en-US/index.ftl")))
    });

    let mut i18n = i18n();

    use_effect(move || {
        let current_locale = *ACTIVE_LOCALE.read();
        let target_langid = current_locale.to_langid();
        i18n.set_language(target_langid);
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved_theme)) = storage.get_item("app_theme") {
                        let theme = match saved_theme.as_str() {
                            "dark" => AppTheme::Dark,
                            _ => AppTheme::Light,
                        };
                        *ACTIVE_THEME.write() = theme;
                        log::info!("Loaded theme from local storage: {}", saved_theme);
                    } else {
                        log::info!("No saved theme found, using default");
                    }
                } else {
                    log::warn!("Local storage is not available");
                }
            }
        }
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let theme = *ACTIVE_THEME.read();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        html.set_attribute("class", theme.as_str()).unwrap();
                    }
                }
            }
        }
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved_locale)) = storage.get_item("app_locale") {
                        let locale = match saved_locale.as_str() {
                            "en" => AppLocale::EN,
                            _ => AppLocale::CN,
                        };
                        *ACTIVE_LOCALE.write() = locale;
                        log::info!("Loaded locale from local storage: {}", saved_locale);
                    } else {
                        log::info!("No saved locale found, using default (CN)");
                        if let Err(e) = storage.set_item("app_locale", AppLocale::default().as_str())
                        {
                            log::error!("Failed to save default locale to local storage: {:?}", e);
                        }
                    }
                } else {
                    log::warn!("Local storage is not available for locale");
                }
            }
        }
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let locale = *ACTIVE_LOCALE.read();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        html.set_attribute("lang", locale.as_str()).unwrap();
                    }
                }
            }
        }
    });

    rsx! {
        title { { t!("common-site-title") } }
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: TW_STYLES }
        div {
            class: "pointer-events-none fixed inset-0 bg-repeat opacity-[0.035] dark:opacity-[0.020]",
            style: "background-image: url({NOISE_IMAGE}); background-size: 180px;",
        }
        AnimatedBird {}
        Router::<Route> {}
    }
}
