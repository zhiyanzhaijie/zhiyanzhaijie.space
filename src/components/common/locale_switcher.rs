use crate::components::common::svgs::I18NSVG;
use crate::components::providers::preference_provider::{
    locale_to_langid, resolve_locale, PreferenceContext,
};
use crate::IO::user;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/theme_bridge.js"::js_apply_lang);

#[component]
pub fn LocaleSwitcher() -> Element {
    let mut preference = use_context::<PreferenceContext>();
    let current_locale = resolve_locale(preference.read().locale.as_deref());
    let mut i18n = i18n();
    let (locale_to_set_on_click, title_for_button) = if current_locale == "en" {
        ("cn", "Switch to Chinese")
    } else {
        ("en", "Switch to English")
    };
    use_effect(move || {
        let locale = current_locale.to_string();
        spawn(async move {
            let _ = js_apply_lang::<()>(locale).await;
        });
    });

    let handle_locale_toggle = move |_| {
        preference.with_mut(|state| {
            state.locale = Some(locale_to_set_on_click.to_string());
        });
        i18n.set_language(locale_to_langid(Some(locale_to_set_on_click)));
        let next_locale = locale_to_set_on_click.to_string();
        spawn(async move {
            let _ = js_apply_lang::<()>(next_locale).await;
        });
        let locale_str = locale_to_set_on_click;
        log::info!("Locale changed to: {}", locale_str);
        spawn(async move {
            if let Err(e) = user::set_locale(locale_str.to_string()).await {
                log::warn!("Failed to save locale to session: {e}");
            }
        });
    };

    rsx! {
        div {
            class: "locale-switcher flex items-center justify-center",
            button {
                class: "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground hover:text-foreground transition-colors",
                onclick: handle_locale_toggle,
                title: "{title_for_button}",
                div {
                    class: "scale-[70%]",
                    I18NSVG { lang: current_locale.to_string() }
                }
            }
        }
    }
}
