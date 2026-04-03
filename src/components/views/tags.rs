use crate::models::post::get_all_posts;
use crate::models::tag::Tag;
use crate::root::{Route, ACTIVE_LOCALE};
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::collections::HashMap;

#[component]
pub fn TagListView() -> Element {
    let current_locale = *ACTIVE_LOCALE.read();
    let current_lang = current_locale.as_str();

    let tag_posts = use_memo(move || {
        let current_locale = *ACTIVE_LOCALE.read();
        let current_lang = current_locale.as_str();

        let posts = get_all_posts();
        let mut posts_map: HashMap<Tag, Vec<crate::models::post::PostMetadata>> = HashMap::new();

        for (post_meta, _) in posts.iter() {
            if post_meta.lang == current_lang {
                if let Some(tags) = &post_meta.tags {
                    for tag in tags {
                        posts_map
                            .entry(tag.clone())
                            .or_insert_with(Vec::new)
                            .push(post_meta.clone());
                    }
                }
            }
        }

        posts_map
    });

    let sorted_tags = use_memo(move || {
        let mut tags: Vec<Tag> = tag_posts.read().keys().cloned().collect();
        tags.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        tags
    });

    let mut active_tag = use_signal(|| None::<Tag>);

    rsx! {
        div {
            class: "flex gap-4 lg:gap-8",

            div {
                class: "flex-1 space-y-6 sm:ml-2 lg:ml-8 lg:space-y-8 mr-44 sm:mr-52",

                div {
                    class: "mb-6 lg:mb-8 pb-4 lg:pb-6 border-b border-border/30",
                    h1 {
                        class: "text-lg sm:text-xl font-medium text-foreground mb-2 sm:mb-3",
                        { t!("page-tag-list-title") }
                    }
                    p {
                        class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                        { t!("page-tag-list-stats",
                             tagCount: sorted_tags.read().len(),
                             postCount: get_all_posts().iter().filter(|(meta, _)| meta.lang == current_lang).count()) }
                    }
                }

                {sorted_tags.read().iter().enumerate().map(|(index, tag)| {
                    let binding = tag_posts.read();
                    let posts = binding.get(tag).unwrap();
                    let mut sorted_posts = posts.clone();
                    sorted_posts.sort_by(|a, b| b.date.cmp(&a.date));

                    rsx! {
                        section {
                            key: "{tag}",
                            id: "tag-{index}",
                            class: "space-y-3 sm:space-y-4",

                            Link {
                                class: "flex items-center space-x-2 sm:space-x-3 pb-2 sm:pb-3 border-b border-border/20",
                                to: Route::BlogByTag { tag: tag.to_string() },
                                h2 {
                                    class: "text-sm sm:text-base font-medium text-foreground hover:underline",
                                    "#{ t!(tag.i18n_key()) }"
                                }
                                span {
                                    class: "flex-shrink-0 text-xs bg-secondary/60 text-secondary-foreground px-2 py-1 rounded-full hover:bg-secondary transition-colors",
                                    "{sorted_posts.len()}"
                                }
                            }

                            div {
                                class: "space-y-1 ml-2 sm:ml-4",
                                {sorted_posts.iter().map(|post_meta| {
                                    let slug_clone = post_meta.slug.clone();
                                    let title_clone = post_meta.title.clone();
                                    let date_clone = post_meta.date.clone();

                                    rsx! {
                                        div {
                                            key: "{slug_clone}",
                                            class: "group flex flex-col sm:flex-row sm:items-center justify-between py-2 px-2 sm:px-3 hover:bg-muted/30 transition-colors duration-150 rounded-sm gap-1 sm:gap-0",

                                            Link {
                                                to: Route::BlogPost { slug: slug_clone.clone() },
                                                class: "text-sm font-medium text-foreground hover:text-primary transition-colors truncate flex-1 min-w-0",
                                                title: "{title_clone}",
                                                "{title_clone}"
                                            }

                                            div {
                                                class: "flex items-center justify-start sm:justify-end space-x-2 sm:space-x-3 text-xs text-muted-foreground flex-shrink-0",
                                                span {
                                                    class: "font-mono hidden sm:inline",
                                                    { t!("page-blog-post-word-count", count: post_meta.word_count) }
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

            if !sorted_tags.is_empty() {
                nav {
                    class: "w-40 sm:w-48 fixed top-36 right-4 sm:right-6 md:right-8 lg:right-12",

                    div {
                        class: "mb-4 text-xs font-medium text-muted-foreground uppercase tracking-wide text-right",
                        { t!("page-tag-list-navigation") }
                    }

                    div {
                        class: "space-y-2 max-h-[calc(100vh-200px)] overflow-y-auto overflow-x-hidden",
                        {sorted_tags.read().iter().enumerate().map(|(index, tag)| {
                            let hue = (index as f32 * 137.5) % 360.0;
                            let binding = tag_posts.read();
                            let post_count = binding.get(tag).unwrap().len();
                            let is_active = active_tag.read().as_ref() == Some(tag);
                            let tag_clone = tag.clone();

                            let color_style = format!("background-color: hsl({}, 65%, 60%)", hue as i32);

                            rsx! {
                                a {
                                    key: "{tag}",
                                    href: "#tag-{index}",
                                    onclick: move |_| {
                                        active_tag.set(Some(tag_clone.clone()));
                                    },
                                    class: "block w-full cursor-pointer group",

                                    div {
                                        class: "relative flex items-center h-10 sm:h-12 overflow-hidden transition-colors duration-300",

                                        if is_active {
                                            div {
                                                class: "absolute -z-1 inset-0 transition-opacity duration-300 bg-[linear-gradient(to_left,color-mix(in_oklch,var(--color-ring),transparent_88%)_0%,transparent_62%)]",
                                            }
                                        }

                                        div {
                                            class: "flex-1 flex flex-col items-end pr-2 sm:pr-3 transition-all duration-300",

                                            div {
                                                class: format!(
                                                    "font-semibold transition-colors duration-300 truncate max-w-24 sm:max-w-28 text-right text-muted-foreground {}",
                                                    if is_active {
                                                        "text-xs sm:text-sm"
                                                    } else {
                                                        "text-xs group-hover:opacity-80"
                                                    }
                                                ),
                                                "{ t!(tag.i18n_key()) }"
                                            }

                                            div {
                                                class: format!(
                                                    "font-medium transition-colors duration-300 {}",
                                                    if is_active {
                                                        "text-muted-foreground text-xs"
                                                    } else {
                                                        "text-muted-foreground/70 text-xs group-hover:text-muted-foreground"
                                                    }
                                                ),
                                                "{post_count}"
                                            }
                                        }

                                        div {
                                            class: format!(
                                                "w-1 h-full transition-all duration-300 {}",
                                                if is_active {
                                                    "shadow-sm"
                                                } else {
                                                    "group-hover:shadow-sm"
                                                }
                                            ),
                                            style: "{color_style}",
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
}
