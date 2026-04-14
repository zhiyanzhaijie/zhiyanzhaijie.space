use crate::components::icons::I18NIcon;
use crate::components::providers::preference_provider::{
    locale_to_langid, resolve_locale, PreferenceContext, PreferenceStoreStoreExt,
};
use crate::IO::user;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/theme_bridge.js"::js_apply_lang);

#[component]
pub fn LocaleSwitcher(#[props(default = false)] compact: bool) -> Element {
    let preference = use_context::<PreferenceContext>();
    let mut locale = preference.locale();
    let current_locale = resolve_locale(locale.read().as_deref());
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
        locale.set(Some(locale_to_set_on_click.to_string()));
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
                class: if compact {
                    "w-5 h-5 flex items-center justify-center focus:outline-none cursor-pointer text-muted-foreground opacity-50 hover:text-foreground hover:opacity-100 transition-colors transition-opacity duration-200"
                } else {
                    "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground opacity-50 hover:text-foreground hover:opacity-100 transition-colors transition-opacity duration-200"
                },
                onclick: handle_locale_toggle,
                title: "{title_for_button}",
                if compact {
                    I18NIcon { lang: current_locale.to_string(), class: "w-3.5 h-3.5" }
                } else {
                    I18NIcon { lang: current_locale.to_string(), class: "w-4 h-4" }
                }
            }
        }
    }
}
