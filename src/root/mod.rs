pub mod layouts;
mod routes;

use crate::components::animated_bird::AnimatedBird;
use crate::impls::{app_runtime, i18n as app_i18n};
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
    use_init_i18n(app_i18n::build_i18n_config);

    let mut i18n = i18n();

    use_effect(move || {
        let current_locale = *ACTIVE_LOCALE.read();
        let target_langid = current_locale.to_langid();
        i18n.set_language(target_langid);
    });

    use_effect(move || {
        if let Some(theme) = app_runtime::load_theme() {
            *ACTIVE_THEME.write() = theme;
        }
    });

    use_effect(move || {
        app_runtime::sync_theme_to_document(*ACTIVE_THEME.read());
    });

    use_effect(move || {
        *ACTIVE_LOCALE.write() = app_runtime::load_locale_or_default();
    });

    use_effect(move || {
        app_runtime::sync_locale_to_document(*ACTIVE_LOCALE.read());
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
