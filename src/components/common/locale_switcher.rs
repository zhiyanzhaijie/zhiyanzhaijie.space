use crate::{components::common::svgs::I18NSVG, AppLocale, ACTIVE_LOCALE};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};

#[component]
pub fn LocaleSwitcher() -> Element {
    let current_locale = *ACTIVE_LOCALE.read();
    let mut i18n = i18n();

    let (locale_to_set_on_click, title_for_button) = match current_locale {
        AppLocale::EN => (AppLocale::CN, t!("locale-switcher-english")),
        AppLocale::CN => (AppLocale::EN, t!("locale-switcher-chinese")),
    };

    let handle_locale_toggle = move |_| {
        *ACTIVE_LOCALE.write() = locale_to_set_on_click;

        i18n.set_language(locale_to_set_on_click.to_langid());

        let locale_str = locale_to_set_on_click.as_str();
        log::info!("Locale changed to: {}", locale_str);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Err(e) = storage.set_item("app_locale", locale_str) {
                        log::error!("Failed to save locale to local storage: {:?}", e);
                    }
                } else {
                    log::warn!("Local storage is not available for locale.");
                }
            } else {
                log::warn!("Window object not found, cannot access local storage for locale.");
            }
        }
    };

    rsx! {
        div {
            class: "locale-switcher p-2",
            button {
                class: "p-1 rounded focus:outline-none cursor-pointer",
                onclick: handle_locale_toggle,
                title: "{title_for_button}",
                I18NSVG { lang: current_locale }
            }
        }
    }
}
