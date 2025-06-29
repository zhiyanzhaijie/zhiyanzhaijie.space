use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::markdown::renderer::MarkdownRenderer;
use crate::models::post::{get_available_languages_for_slug, get_post_by_slug_and_lang};
use crate::routes::Route;
use crate::ACTIVE_LOCALE;

#[component]
pub fn BlogPost(slug: String) -> Element {
    // 直接在 use_memo 内部读取 ACTIVE_LOCALE，并实现回退逻辑
    let post = use_memo(move || {
        let current_locale = *ACTIVE_LOCALE.read();
        let active_lang = current_locale.as_str();

        // 首先尝试获取当前语言的文章
        if let Some(post) = get_post_by_slug_and_lang(&slug, active_lang) {
            return Some(post);
        }

        // 如果当前语言不存在，获取该 slug 的所有可用语言
        let available_languages = get_available_languages_for_slug(&slug);

        // 按优先级回退：英文 -> 第一个可用语言
        let fallback_lang = if available_languages.contains(&"en".to_string()) {
            "en"
        } else if let Some(first_lang) = available_languages.first() {
            first_lang.as_str()
        } else {
            // 如果没有任何可用语言，返回 None
            return None;
        };

        // 返回回退语言的文章
        get_post_by_slug_and_lang(&slug, fallback_lang)
    });

    match post() {
        Some((meta, content)) => {
            rsx! {
                div { class: "w-full min-h-screen px-2 sm:px-4 py-4 overflow-hidden overflow-y-auto",

                    article { class: "container lg:ml-8 sm:ml-2 max-w-6xl",
                        // Article header
                        header { class: "mb-6 sm:mb-8 pb-4 sm:pb-6 border-b border-border/30",
                            h1 { class: "text-xl sm:text-2xl lg:text-3xl font-bold text-foreground mb-3 sm:mb-4 leading-tight", {meta.title.clone()} }


                            div { class: "flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4 text-sm text-muted-foreground",
                                p { class: "flex items-center",
                                    { t!("page-blog-post-published") } " {meta.date.clone()}"
                                }

                                // Word count for larger screens
                                span { class: "hidden sm:inline-block text-xs",
                                    "•"
                                }
                                span { class: "hidden sm:inline-block text-xs",
                                    { t!("page-blog-post-word-count", count: meta.word_count) }
                                }
                            }

                            // Tags display
                            if let Some(ref tags) = meta.tags {
                                div {
                                    class: "flex flex-wrap gap-2 mt-3 sm:mt-4",
                                    for tag in tags.iter() {
                                        Link {
                                            to: Route::BlogByTag { tag: tag.to_string() },
                                            class: "inline-flex items-center px-2 py-1 text-xs bg-secondary/60 text-secondary-foreground rounded-full hover:bg-secondary transition-colors",
                                            "#{ t!(tag.i18n_key()) }"
                                        }
                                    }
                                }
                            }
                        }

                        // Article content
                        div { class: "prose prose-sm sm:prose-base lg:prose-lg max-w-none prose-slate dark:prose-invert",
                            MarkdownRenderer { content: content.clone() }
                        }
                    }

                    // Navigation
                    nav { class: "mt-8 sm:mt-12 pt-6 sm:pt-8 border-t border-border/30",
                        div { class: "container max-w-4xl hover:underline",
                            Link {
                                class: "inline-flex items-center gap-2 text-sm sm:text-base text-primary hover:text-primary/80 transition-colors",
                                to: Route::BlogList { },
                                { t!("common-button-back") }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "container  px-4 py-8 sm:py-16 text-center",
                    h1 { class: "text-xl sm:text-2xl lg:text-3xl font-bold text-foreground mb-4 leading-tight", { t!("page-not-found-title") } }
                    p { class: "text-sm sm:text-base text-muted-foreground mb-6 max-w-md mx-auto", { t!("page-not-found-message") } }
                    Link {
                        class: "inline-flex items-center justify-center px-4 py-2 text-sm sm:text-base font-medium text-primary-foreground bg-primary rounded-md hover:bg-primary/90 transition-colors min-h-[44px]",
                        to: Route::BlogList { },
                        { t!("page-not-found-back-home") }
                    }
                }
            }
        }
    }
}
