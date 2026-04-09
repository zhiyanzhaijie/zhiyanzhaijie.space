use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::providers::preference_provider::{resolve_locale, PreferenceContext};
use crate::root::Route;
use crate::IO::blog;
use dioxus::prelude::*;

#[component]
pub fn TagListView() -> Element {
    let preference = use_context::<PreferenceContext>();
    let tag_groups_fut = use_server_future(move || {
        let current_lang = resolve_locale(preference.read().locale.as_deref()).to_string();
        async move { blog::get_tag_groups(current_lang).await }
    })?;
    let posts_fut = use_server_future(move || {
        let current_lang = resolve_locale(preference.read().locale.as_deref()).to_string();
        async move { blog::get_posts_by_lang(current_lang).await }
    })?;

    let tag_groups = match tag_groups_fut() {
        Some(Ok(groups)) => groups,
        _ => Vec::new(),
    };
    let article_count = match posts_fut() {
        Some(Ok(posts)) => posts.len(),
        _ => 0,
    };

    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "space-y-8",

                div {
                    h1 {
                        class: "text-xl sm:text-2xl font-semibold tracking-tight text-foreground mb-2",
                        "Tag"
                    }
                    p {
                        class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                        "{tag_groups.len()} tags, {article_count} articles"
                    }
                }

                {tag_groups.iter().enumerate().map(|(index, group)| {
                    let tag = group.tag.clone();
                    let tag_id = tag.id.clone();
                    let tag_label = tag.label.clone();
                    let sorted_posts = group.posts.clone();

                    rsx! {
                        section {
                            key: "{tag}",
                            id: "tag-{index}",
                            class: "space-y-3",

                            Link {
                                class: "flex items-center space-x-2 sm:space-x-3",
                                to: Route::BlogByTag { tag: tag_id.clone() },
                                h2 {
                                    class: "text-sm sm:text-base font-medium text-foreground hover:underline underline-offset-4",
                                    "#{ tag_label }"
                                }
                                span {
                                    class: "flex-shrink-0 text-xs text-muted-foreground",
                                    "{sorted_posts.len()}"
                                }
                            }

                            div {
                                class: "space-y-1 ml-2",
                                {sorted_posts.iter().map(|post_meta| {
                                    let slug_clone = post_meta.slug.clone();
                                    let title_clone = post_meta.title.clone();
                                    let date_clone = post_meta.date.clone();

                                    rsx! {
                                        div {
                                            key: "{slug_clone}",
                                            class: "group flex flex-col sm:flex-row sm:items-center justify-between py-2 gap-1 sm:gap-0",

                                            Link {
                                                to: Route::BlogPost { slug: slug_clone.clone() },
                                                class: "text-sm font-medium text-foreground hover:underline underline-offset-4 transition-colors truncate flex-1 min-w-0",
                                                title: "{title_clone}",
                                                "{title_clone}"
                                            }

                                            div {
                                                class: "flex items-center justify-start sm:justify-end space-x-2 sm:space-x-3 text-xs text-muted-foreground flex-shrink-0",
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
                })}
            }
        }
    }
}
