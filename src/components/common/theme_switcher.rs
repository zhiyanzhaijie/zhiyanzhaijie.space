use crate::{
    components::common::svgs::{MoonSVG, SunSVG},
    root::AppTheme,
    root::ACTIVE_THEME,
};
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher(#[props(default = false)] is_mobile: bool) -> Element {

    let handle_theme_toggle = move |_| {
        let current_theme = *ACTIVE_THEME.read();
        let new_theme = match current_theme {
            AppTheme::Light => AppTheme::Dark,
            AppTheme::Dark => AppTheme::Light,
        };

        *ACTIVE_THEME.write() = new_theme;

        let theme_str = match new_theme {
            AppTheme::Light => "light",
            AppTheme::Dark => "dark",
        };

        log::info!("Theme changed to: {}", theme_str);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Err(e) = storage.set_item("app_theme", theme_str) {
                        log::error!("Failed to save theme to local storage: {:?}", e);
                    }
                }
            }
        }
    };

    let current_theme = *ACTIVE_THEME.read();

    let title_text = match current_theme {
        AppTheme::Light => "Switch to dark mode",
        AppTheme::Dark => "Switch to light mode",
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
                        AppTheme::Light => rsx! { MoonSVG {} },
                        AppTheme::Dark => rsx! { SunSVG {} },
                    }
                }
            }
        }
    }
}
