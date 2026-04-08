use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::markdown::hooks::use_markdown_components;
use crate::components::markdown::renderer::MarkdownRenderer;
use crate::components::providers::preference_provider::{resolve_locale, PreferenceContext};
use crate::models::post::{get_available_languages_for_slug, get_post_by_slug_and_lang};
use crate::root::Route;
use crate::utils::markdown_toc::inject_heading_anchors_and_collect_toc;
use dioxus::document::Stylesheet;
use dioxus::prelude::*;
const MARKDOWN_CSS: Asset = asset!("/assets/markdown.css");

#[component]
pub fn BlogPostView(slug: String) -> Element {
    let markdown_components = use_markdown_components();
    let preference = use_context::<PreferenceContext>();

    let post = use_memo(move || {
        let active_lang = resolve_locale(preference.read().locale.as_deref());

        if let Some(post) = get_post_by_slug_and_lang(&slug, active_lang) {
            return Some(post);
        }

        let available_languages = get_available_languages_for_slug(&slug);

        let fallback_lang = if available_languages.contains(&"en".to_string()) {
            "en"
        } else if let Some(first_lang) = available_languages.first() {
            first_lang.as_str()
        } else {
            return None;
        };

        get_post_by_slug_and_lang(&slug, fallback_lang)
    });

    match post() {
        Some((meta, content)) => {
            let (content_with_anchors, _) = inject_heading_anchors_and_collect_toc(&content);
            rsx! {
                LayoutCell {
                    padding: LayoutCellPadding::Normal,
                    Stylesheet { href: MARKDOWN_CSS }
                    article {
                        header { class: "mb-7",
                            h1 { class: "text-2xl sm:text-3xl font-semibold tracking-tight text-foreground mb-3 leading-tight", {meta.title.clone()} }


                            div { class: "flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4 text-sm text-muted-foreground",
                                p { class: "flex items-center",
                                    "Published on {meta.date.clone()}"
                                }

                                span { class: "hidden sm:inline-block text-xs",
                                    "•"
                                }
                                span { class: "hidden sm:inline-block text-xs",
                                    "{meta.word_count} words"
                                }
                            }

                            if let Some(ref tags) = meta.tags {
                                div {
                                    class: "flex flex-wrap gap-2 mt-3 sm:mt-4",
                                    for tag in tags.iter() {
                                        Link {
                                            to: Route::BlogByTag { tag: tag.to_string() },
                                            class: "inline-flex items-center text-xs text-muted-foreground hover:text-foreground transition-colors",
                                            "#{ tag.label_en() }"
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "prose prose-sm sm:prose-base lg:prose-lg max-w-none prose-slate dark:prose-invert",
                            MarkdownRenderer {
                                content: content_with_anchors.clone(),
                                components: markdown_components,
                            }
                        }

                        nav { class: "mt-8 sm:mt-12 pt-6 sm:pt-8",
                            div { class: "hover:underline",
                                Link {
                                    class: "inline-flex items-center gap-2 text-sm sm:text-base text-muted-foreground hover:text-foreground transition-colors",
                                    to: Route::BlogList { },
                                    "Back"
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                LayoutCell {
                    padding: LayoutCellPadding::Normal,
                    div { class: "py-10 sm:py-16 text-center",
                        h1 { class: "text-xl sm:text-2xl lg:text-3xl font-semibold tracking-tight text-foreground mb-4 leading-tight", "Page Not Found" }
                        p { class: "text-sm sm:text-base text-muted-foreground mb-6 max-w-md mx-auto", "Sorry, we couldn't find the page you're looking for." }
                        Link {
                            class: "inline-flex items-center justify-center px-2 py-2 text-sm sm:text-base font-medium text-muted-foreground hover:text-foreground transition-colors min-h-[44px] underline-offset-4 hover:underline",
                            to: Route::BlogList { },
                            "Back to Home"
                        }
                    }
                }
            }
        }
    }
}
