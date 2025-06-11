use crate::{AppTheme, ACTIVE_THEME};
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let (icon_element_to_display, theme_to_set_on_click, title_for_button) = {
        let current_theme_value = ACTIVE_THEME.read();
        match *current_theme_value {
            AppTheme::Light => (rsx!(SunSVG {}), AppTheme::Dark, "Switch to Dark Theme"),
            AppTheme::Dark => (rsx!(MoonSVG {}), AppTheme::Light, "Switch to Light Theme"),
        }
    };

    let handle_theme_toggle = move |_| {
        *ACTIVE_THEME.write() = theme_to_set_on_click;

        let theme_str = match theme_to_set_on_click {
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
                } else {
                    log::warn!("Local storage is not available.");
                }
            } else {
                log::warn!("Window object not found, cannot access local storage.");
            }
        }
    };

    rsx! {
        div {
            class: "theme-switcher p-2", // Container for the button
            button {
                class: "p-2 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 hover:bg-gray-200 dark:hover:bg-gray-700",
                onclick: handle_theme_toggle,
                title: title_for_button,
                {icon_element_to_display}
            }
        }
    }
}

#[component]
pub fn SunSVG() -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            path {
                fill: "currentColor",
                fill_rule: "evenodd",
                d: "M12 2a1 1 0 0 1 1 1v1a1 1 0 1 1-2 0V3a1 1 0 0 1 1-1M2 12a1 1 0 0 1 1-1h1a1 1 0 1 1 0 2H3a1 1 0 0 1-1-1m17 0a1 1 0 0 1 1-1h1a1 1 0 1 1 0 2h-1a1 1 0 0 1-1-1m-6 8a1 1 0 1 0-2 0v1a1 1 0 1 0 2 0zm5.364-3.05a1 1 0 1 0-1.414 1.414l.707.707a1 1 0 0 0 1.414-1.414zM4.929 4.929a1 1 0 0 1 1.414 0l.707.707A1 1 0 0 1 5.636 7.05l-.707-.707a1 1 0 0 1 0-1.414M7.05 18.364a1 1 0 1 0-1.414-1.414l-.707.707a1 1 0 1 0 1.414 1.414zM19.071 4.929a1 1 0 0 1 0 1.414l-.707.707a1 1 0 1 1-1.414-1.414l.707-.707a1 1 0 0 1 1.414 0M7 12a5 5 0 1 1 10 0a5 5 0 0 1-10 0",
                clip_rule: "evenodd"
            }
        }
    )
}

#[component]
pub fn MoonSVG() -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            path {
                fill: "none", // Moon icon is often an outline
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
                d: "M12 21a9 9 0 0 0 8.997-9.252a7 7 0 0 1-10.371-8.643A9 9 0 0 0 12 21"
            }
        }
    )
}
