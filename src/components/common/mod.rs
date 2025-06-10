use crate::{AppTheme, ACTIVE_THEME};
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let current_theme = ACTIVE_THEME.read();

    rsx! {
        div {
            class: "theme-switcher p-4",
            span { class: "mr-2", "Select Theme:" }
            select {
                // 根据当前主题动态添加一些 Tailwind 样式
                class: "p-2 rounded border bg-gray-100 dark:bg-gray-700 dark:text-white focus:ring-2",
                oninput: move |event| {
                    let new_theme_str = event.value();
                    let new_theme = match new_theme_str.as_str() {
                        "light" => AppTheme::Light,
                        "dark" => AppTheme::Dark,
                        _ => AppTheme::default(), // 安全回退
                    };

                    *ACTIVE_THEME.write() = new_theme; // 更新全局主题状态
                    log::info!("Theme changed to: {}", new_theme_str);
                    // (可选) 将选择的主题保存到 localStorage
                    #[cfg(target_arch = "wasm32")]
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let storage = window.local_storage().expect("no local storage").expect("local storage is not available");
                        if let Err(e) = storage.set_item("app_theme", &new_theme_str) {
                        }
                    }
                },
                option { value: "light", selected: *current_theme == AppTheme::Light, "Light" }
                option { value: "dark", selected: *current_theme == AppTheme::Dark, "Dark" }
            }
        }
    }
}
