use crate::{
    components::common::svgs::{MoonSVG, SunSVG},
    AppTheme, ACTIVE_THEME,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut rotation_count = use_signal(|| 0);

    let handle_theme_toggle = move |_| {
        let current_theme = *ACTIVE_THEME.read();
        let new_theme = match current_theme {
            AppTheme::Light => AppTheme::Dark,
            AppTheme::Dark => AppTheme::Light,
        };

        *ACTIVE_THEME.write() = new_theme;
        rotation_count.set(rotation_count() + 1);

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
        AppTheme::Light => t!("theme-switcher-light"),
        AppTheme::Dark => t!("theme-switcher-dark"),
    };

    let rotation_degrees = - rotation_count() * 180;

    rsx! {
        div {
            class: "relative w-12 h-12 p-2 mr-4",

            button {
                class: "w-full h-full bg-transparent border-none rounded-md cursor-pointer focus:outline-none",
                title: "{title_text}",

                div {
                    class: "flex w-18 h-6 overflow-hidden justify-center relative",

                    div {
                        class: "w-6 h-16 flex flex-col transition-transform duration-800 ease-in-ou",
                        style: "transform: rotate({rotation_degrees}deg);",
                        onclick: handle_theme_toggle,

                        div {
                            class: "w-6 h-6 flex-shrink-0 flex -rotate-90 items-center justify-center drop-shadow-lg",
                            MoonSVG {}
                        }
                        div {
                          class: "w-4 h-4 flex-shrink-0"
                        }

                        div {
                            class: "w-6 h-6 flex-shrink-0 flex items-center justify-center drop-shadow-lg",
                            SunSVG {}
                        }
                    }
                }
            }
        }
    }
}
