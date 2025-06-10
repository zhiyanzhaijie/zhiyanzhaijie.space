use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use log::Level;
use web_sys::window;

mod components;
mod models;
mod pages;
mod routes;
mod utils;

use crate::components::common::ThemeSwitcher;
use crate::routes::Route;

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

static ACTIVE_THEME: GlobalSignal<AppTheme> = Signal::global(AppTheme::default);

#[allow(non_snake_case)]
fn App() -> Element {
    use_effect(move || {
        let theme = *ACTIVE_THEME.read();
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    html.set_attribute("class", theme.as_str()).unwrap();
                }
            }
        }
    });

    const TW_STYLES: Asset = asset!("/assets/tailwind.css");

    rsx! {
        title { "我的博客吗" }
        Stylesheet {
          href: TW_STYLES
        }
        main {
            class: "w-screen h-screen flex flex-col overflow-hidden",
            ThemeSwitcher {}
            "main"
            Router::<Route> {}
        }
    }
}

#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        match console_log::init_with_level(Level::Debug) {
            Ok(_) => {
                log::info!("console_log initialized successfully at level: Debug");
            }
            Err(e) => {
                eprintln!("Error initializing console_log: {:?}", e);
            }
        }
    }
    launch(App);
}
