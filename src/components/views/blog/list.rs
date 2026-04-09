use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::providers::preference_provider::{resolve_locale, PreferenceContext};
use crate::root::Route;
use crate::IO::blog;
use dioxus::prelude::*;

#[component]
pub fn BlogListView() -> Element {
    let preference = use_context::<PreferenceContext>();
    let posts_fut = use_server_future(move || {
        let current_lang = resolve_locale(preference.read().locale.as_deref()).to_string();
        async move { blog::get_posts_by_lang(current_lang).await }
    })?;

    let posts = match posts_fut() {
        Some(Ok(posts)) => posts,
        _ => Vec::new(),
    };

    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "space-y-1",
                if posts.is_empty() {
                    div {
                        class: "text-center py-12 text-muted-foreground",
                        "No articles available"
                    }
                } else {
                    {posts.iter().map(|post_meta| {
                        let slug_clone = post_meta.slug.clone();
                        let title_clone = post_meta.title.clone();
                        let date_clone = post_meta.date.clone();
                        let tags_clone = post_meta.tags.clone();

                        rsx! {
                            div {
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
                                        if let Some(first_tag) = tags.first() {
                                            Link {
                                                to: Route::BlogByTag { tag: first_tag.to_string() },
                                                class: "flex-shrink-0 text-xs text-muted-foreground hover:text-foreground transition-colors order-2",
                                                "#{ first_tag.label }"
                                            }
                                        }
                                    }
                                }

                                div {
                                    class: "flex items-center justify-start sm:justify-end space-x-3 sm:space-x-4 text-xs text-muted-foreground flex-shrink-0",
                                    span {
                                        class: "font-mono hidden sm:inline",
                                        "{post_meta.word_count} words"
                                    }
                                    span {
                                        class: "font-mono",
                                        "{date_clone}"
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
