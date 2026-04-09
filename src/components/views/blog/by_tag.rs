use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::providers::preference_provider::{resolve_locale, PreferenceContext};
use crate::root::Route;
use crate::IO::blog;
use dioxus::prelude::*;

#[component]
pub fn BlogByTagView(tag: String) -> Element {
    let preference = use_context::<PreferenceContext>();
    let tag_id = tag.trim().to_lowercase();
    let query_tag_id = tag_id.clone();

    let posts_fut = use_server_future(move || {
        let current_lang = resolve_locale(preference.read().locale.as_deref()).to_string();
        let tag = query_tag_id.clone();
        async move { blog::get_posts_by_tag(tag, current_lang).await }
    })?;

    let sorted_posts = match posts_fut() {
        Some(Ok(posts)) => posts,
        _ => Vec::new(),
    };
    let tag_label = sorted_posts
        .iter()
        .find_map(|post_meta| {
            post_meta
                .tags
                .as_ref()
                .and_then(|tags| {
                    tags.iter()
                        .find(|item| item.id.as_str() == tag_id.as_str())
                })
                .map(|item| item.label.clone())
        })
        .unwrap_or_else(|| tag_id.clone());

    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "space-y-1",

                div {
                    class: "mb-5",
                    nav {
                        class: "mb-2 sm:mb-3 text-xs sm:text-sm text-muted-foreground",
                        span { class: "mx-1 sm:mx-2", "/" }
                        Link {
                            to: Route::TagList {},
                            class: "hover:text-foreground transition-colors whitespace-nowrap",
                            "Tag"
                        }
                        span { class: "mx-1 sm:mx-2", "/" }
                        span { class: "text-foreground whitespace-nowrap", "{tag_label}" }
                    }
                    h1 {
                        class: "text-xl sm:text-2xl font-semibold tracking-tight text-foreground mb-2 leading-tight",
                        "Articles with \"{tag_label}\""
                    }
                    p {
                        class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                        "Related to {tag_label}"
                    }
                }

                if sorted_posts.is_empty() {
                    div {
                        class: "text-center py-8 sm:py-12 text-muted-foreground space-y-3 max-w-md mx-auto",
                        div {
                            class: "text-sm sm:text-base",
                            "No articles found for this tag"
                        }
                        Link {
                            to: Route::TagList {},
                            class: "inline-flex items-center text-sm text-primary hover:text-primary/80 transition-colors min-h-[44px] justify-center",
                            "← "
                            "Tag"
                        }
                    }
                } else {
                    {sorted_posts.into_iter().map(|post_meta| {
                        let slug_clone = post_meta.slug.clone();
                        let title_clone = post_meta.title.clone();
                        let date_clone = post_meta.date.clone();
                        let tags_clone = post_meta.tags.clone();

                        rsx! {
                            article {
                                key: "{slug_clone}",
                                class: "group flex flex-col sm:flex-row sm:items-center justify-between py-3 gap-2 sm:gap-0",
                                div {
                                    class: "flex items-center space-x-2 sm:space-x-3 min-w-0 flex-1 flex-wrap gap-y-1 sm:gap-y-0",
                                    Link {
                                        to: Route::BlogPost { slug: slug_clone.clone() },
                                        class: "text-sm font-medium text-foreground hover:underline underline-offset-4 transition-colors truncate order-1 flex-shrink min-w-0",
                                        title: "{title_clone}",
                                        "{title_clone}"
                                    }
                                    if let Some(tags) = &tags_clone {
                                        div {
                                            class: "flex items-center space-x-1 flex-shrink-0 order-2",
                                            {tags.iter()
                                                .filter(|tag| tag.id.as_str() != tag_id.as_str())
                                                .take(2)
                                                .map(|other_tag| rsx! {
                                                    Link {
                                                        key: "{other_tag}",
                                                        to: Route::BlogByTag { tag: other_tag.id.clone() },
                                                        class: "flex-shrink-0 text-xs text-muted-foreground hover:text-foreground transition-colors",
                                                        title: "View tag",
                                                        "#{ other_tag.label }"
                                                    }
                                                })
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex items-center justify-start sm:justify-end space-x-3 sm:space-x-4 text-xs text-muted-foreground flex-shrink-0",
                                    span {
                                        class: "font-mono hidden sm:inline",
                                        title: "Word count",
                                        "{post_meta.word_count} words"
                                    }
                                    span {
                                        class: "font-mono",
                                        title: "Published date",
                                        "{date_clone}"
                                    }
                                }
                            }
                        }
                    })}
                    div {
                        class: "pt-4 mt-4 text-center",
                        Link {
                            to: Route::TagList {},
                            class: "inline-flex items-center text-sm text-muted-foreground hover:text-foreground transition-colors min-h-[44px] justify-center",
                            "← "
                            "Tag"
                        }
                    }
                }
            }
        }
    }
}
