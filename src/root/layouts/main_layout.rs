use crate::{
    components::common::{
        locale_switcher::LocaleSwitcher, svgs::LogoSVG, theme_switcher::ThemeSwitcher,
    },
    models::post::{get_available_languages_for_slug, get_post_by_slug_and_lang},
    root::Route,
    root::ACTIVE_LOCALE,
    utils::markdown_toc::collect_toc_items,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

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
pub fn MainLayout() -> Element {
    let current_route = use_route::<Route>();
    let current_locale = *ACTIVE_LOCALE.read();
    let current_lang = current_locale.as_str();

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

                    aside {
                        class: "w-48 shrink-0 sticky top-8 space-y-4",
                        Link {
                            to: Route::BlogList {},
                            class: "inline-flex items-center",
                            LogoSVG {}
                        }

                        div {
                            input {
                                class: "w-full h-8 px-2 text-sm bg-transparent text-muted-foreground focus:outline-none",
                                r#type: "text",
                                placeholder: "Search..."
                            }
                        }

                        div {
                            class: "flex items-center gap-1",
                            ThemeSwitcher { is_mobile: false }
                            LocaleSwitcher {}
                        }
                        nav {
                            class: "space-y-2 text-sm",
                            Link {
                                to: Route::BlogList {},
                                class: format!(
                                    "block transition-colors {}",
                                    if matches!(current_route, Route::BlogList { .. } | Route::BlogPost { .. }) {
                                        "text-foreground font-medium"
                                    } else {
                                        "text-muted-foreground hover:text-foreground"
                                    }
                                ),
                                { t!("main-layout-nav-articles") }
                            }
                            Link {
                                to: Route::TagList {},
                                class: format!(
                                    "block transition-colors {}",
                                    if matches!(current_route, Route::TagList { .. } | Route::BlogByTag { .. }) {
                                        "text-foreground font-medium"
                                    } else {
                                        "text-muted-foreground hover:text-foreground"
                                    }
                                ),
                                { t!("main-layout-nav-tags") }
                            }
                            Link {
                                to: Route::About {},
                                class: format!(
                                    "block transition-colors {}",
                                    if matches!(current_route, Route::About { .. }) {
                                        "text-foreground font-medium"
                                    } else {
                                        "text-muted-foreground hover:text-foreground"
                                    }
                                ),
                                { t!("main-layout-nav-about") }
                            }
                        }
                    }

                    section {
                        class: "flex-1 min-w-0 overflow-x-hidden text-muted-foreground",
                        if matches!(current_route, Route::BlogPost { .. }) {
                            div {
                                class: "flex items-start gap-8",
                                div {
                                    class: "flex-1 min-w-0 max-w-2xl",
                                    Outlet::<Route> {}
                                }
                                if !toc_items.is_empty() {
                                    aside {
                                        class: "w-52 shrink-0 sticky top-20",
                                        div {
                                            class: "space-y-2",
                                            div { class: "text-xs text-muted-foreground", "On This Page" }
                                            nav {
                                                class: "space-y-1",
                                                for item in toc_items.iter() {
                                                    a {
                                                        href: "#{item.id}",
                                                        class: format!(
                                                            "block text-xs transition-colors {}",
                                                            if item.level >= 3 {
                                                                "pl-3 text-muted-foreground hover:text-foreground"
                                                            } else {
                                                                "text-muted-foreground hover:text-foreground"
                                                            }
                                                        ),
                                                        "{item.title}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            div {
                                class: "max-w-2xl",
                                Outlet::<Route> {}
                            }
                        }
                    }
                }
            }
        }
    }
}
