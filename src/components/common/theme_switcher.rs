use crate::IO::user;
use crate::{
    components::common::svgs::{MoonSVG, SunSVG},
    components::providers::preference_provider::{resolve_theme, PreferenceContext},
};
use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/theme_bridge.js"::js_apply_theme);

#[component]
pub fn ThemeSwitcher(#[props(default = false)] is_mobile: bool) -> Element {
    let mut preference = use_context::<PreferenceContext>();

    let handle_theme_toggle = move |_| {
        let current_theme = resolve_theme(preference.read().theme.as_deref());
        let new_theme = match current_theme {
            "light" => "dark",
            _ => "light",
        };
        preference.with_mut(|state| {
            state.theme = Some(new_theme.to_string());
        });
        let next_theme = new_theme.to_string();
        spawn(async move {
            let _ = js_apply_theme::<()>(next_theme).await;
        });
        let theme_str = new_theme;

        log::info!("Theme changed to: {}", theme_str);
        spawn(async move {
            if let Err(e) = user::set_theme(theme_str.to_string()).await {
                log::warn!("Failed to save theme to session: {e}");
            }
        });
    };

    let current_theme = resolve_theme(preference.read().theme.as_deref());

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
    let button_class = if is_mobile {
        "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground hover:text-foreground transition-colors"
    } else {
        "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground hover:text-foreground transition-colors"
    };

    rsx! {
        div {
            class: "flex items-center justify-center",
            button {
                class: "{button_class}",
                title: "{title_text}",
                onclick: handle_theme_toggle,
                div {
                    class: "scale-[70%]",
                    match current_theme {
                        "light" => rsx! { MoonSVG {} },
                        _ => rsx! { SunSVG {} },
                    }
                }
            }
        }
    }
}
