use crate::IO::user;
use crate::{
    components::icons::{MoonIcon, SunIcon},
    components::providers::preference_provider::{
        resolve_theme, PreferenceContext, PreferenceStoreStoreExt,
    },
};
use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/theme_bridge.js"::js_apply_theme);

#[component]
pub fn ThemeSwitcher(
    #[props(default = false)] is_mobile: bool,
    #[props(default = false)] compact: bool,
) -> Element {
    let preference = use_context::<PreferenceContext>();
    let mut theme = preference.theme();

    let handle_theme_toggle = move |_| {
        let current_theme = resolve_theme(theme.read().as_deref());
        let new_theme = match current_theme {
            "light" => "dark",
            _ => "light",
        };
        theme.set(Some(new_theme.to_string()));
        let next_theme = new_theme.to_string();
        spawn(async move {
            let _ = js_apply_theme::<()>(next_theme).await;
        });
        let theme_str = new_theme;

        spawn(async move {
            if let Err(e) = user::set_theme(theme_str.to_string()).await {
                log::warn!("Failed to save theme to session: {e}");
            }
        });
    };

    let current_theme = resolve_theme(theme.read().as_deref());

    use_effect(move || {
        let theme = current_theme.to_string();
        spawn(async move {
            let _ = js_apply_theme::<()>(theme).await;
        });
    });
    let title_text = match current_theme {
        "light" => "Switch to dark mode",
        _ => "Switch to light mode",
    };
    let button_class = if compact {
        "w-5 h-5 flex items-center justify-center focus:outline-none cursor-pointer text-muted-foreground opacity-50 hover:text-foreground hover:opacity-100 transition-colors transition-opacity duration-200"
    } else {
        "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground opacity-50 hover:text-foreground hover:opacity-100 transition-colors transition-opacity duration-200"
    };

    rsx! {
        div {
            class: "flex items-center justify-center",
            button {
                class: "{button_class}",
                title: "{title_text}",
                onclick: handle_theme_toggle,
                if compact {
                    match current_theme {
                        "light" => rsx! { MoonIcon { class: "w-3.5 h-3.5" } },
                        _ => rsx! { SunIcon { class: "w-3.5 h-3.5" } },
                    }
                } else {
                    match current_theme {
                        "light" => rsx! { MoonIcon { class: "w-4 h-4" } },
                        _ => rsx! { SunIcon { class: "w-4 h-4" } },
                    }
                }
            }
        }
    }
}
