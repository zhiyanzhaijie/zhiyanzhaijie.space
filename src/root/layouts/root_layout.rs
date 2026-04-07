use crate::{
    components::providers::preference_provider::{resolve_locale, PreferenceContext},
    components::layout::root::{content::RootLayoutContent, sidebar::RootAsidebar},
    models::post::{get_available_languages_for_slug, get_post_by_slug_and_lang},
    root::Route,
    utils::markdown_toc::collect_toc_items,
};
use dioxus::prelude::*;

fn load_post_content_with_fallback(slug: &str, lang: &str) -> Option<String> {
    if let Some((_, content)) = get_post_by_slug_and_lang(slug, lang) {
        return Some(content);
    }

    let available_languages = get_available_languages_for_slug(slug);
    if available_languages.is_empty() {
        return None;
    }

    let fallback_lang = if available_languages.iter().any(|l| l == "en") {
        "en".to_string()
    } else {
        available_languages[0].clone()
    };

    get_post_by_slug_and_lang(slug, &fallback_lang).map(|(_, content)| content)
}

#[component]
pub fn RootLayout() -> Element {
    let current_route = use_route::<Route>();
    let preference = use_context::<PreferenceContext>();
    let current_lang = resolve_locale(preference.read().locale.as_deref());

    let toc_items = if let Route::BlogPost { slug } = &current_route {
        if let Some(content) = load_post_content_with_fallback(slug, current_lang) {
            collect_toc_items(&content)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    rsx! {
        div {
            class: "min-h-screen bg-background text-foreground font-sans",
            main {
                class: "max-w-6xl mx-auto px-4 sm:px-6 md:px-8 py-8 sm:py-10 relative z-10",
                div {
                    class: "flex items-start gap-8",
                    RootAsidebar { current_route: current_route.clone() }
                    RootLayoutContent {
                        current_route: current_route,
                        toc_items: toc_items,
                    }
                }
            }
        }
    }
}
