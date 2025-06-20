use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
use unic_langid::{langid, LanguageIdentifier};
use web_sys::window;

mod components;
mod models;
mod pages;
mod routes;
mod utils;

use crate::routes::Route;
use components::animated_bird::AnimatedBird;

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

static ACTIVE_THEME: GlobalSignal<AppTheme> = Signal::global(AppTheme::default);

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

static ACTIVE_LOCALE: GlobalSignal<AppLocale> = Signal::global(AppLocale::default);

#[allow(non_snake_case)]
fn App() -> Element {
    // Initialize i18n with layered component structure
    use_init_i18n(|| {
        I18nConfig::new(langid!("zh-CN"))
            .with_locale((langid!("zh-CN"), include_str!("./i18n/zh-CN/index.ftl")))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US/index.ftl")))
    });

    let mut i18n = i18n();

    // Sync i18n with AppLocale
    use_effect(move || {
        let current_locale = *ACTIVE_LOCALE.read();
        let target_langid = current_locale.to_langid();
        i18n.set_language(target_langid);
    });

    // 初始化时从本地存储读取主题设置
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

    // 监听主题变化并应用到HTML元素
    use_effect(move || {
        let theme = *ACTIVE_THEME.read();
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    html.set_attribute("class", theme.as_str()).unwrap();
                }
            }
        }
    });

    // 初始化时从本地存储读取区域设置
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved_locale)) = storage.get_item("app_locale") {
                        let locale = match saved_locale.as_str() {
                            "en" => AppLocale::EN,
                            _ => AppLocale::CN, // 默认为中文
                        };
                        *ACTIVE_LOCALE.write() = locale;
                        log::info!("Loaded locale from local storage: {}", saved_locale);
                    } else {
                        log::info!("No saved locale found, using default (CN)");
                        // 如果没有保存的设置，则将默认值写入本地存储
                        if let Err(e) =
                            storage.set_item("app_locale", AppLocale::default().as_str())
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

    // 监听区域设置变化并应用到HTML lang属性
    use_effect(move || {
        let locale = *ACTIVE_LOCALE.read();
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    html.set_attribute("lang", locale.as_str()).unwrap();
                }
            }
        }
    });

    const TW_STYLES: Asset = asset!("/assets/tailwind.css");

    rsx! {
        title { { t!("common-site-title") } }
        Stylesheet {
          href: TW_STYLES
        }
        // Animated ASCII Art Bird Logo Silhouette Background
        AnimatedBird {}
        Router::<Route> {}
    }
}

#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        match console_log::init_with_level(log::Level::Debug) {
            Ok(_) => {
                log::info!("console_log initialized successfully at level: Debug");
            }
            Err(e) => {
                eprintln!("Error initializing console_log: {:?}", e);
            }
        }
    }
    launch(App);
}
