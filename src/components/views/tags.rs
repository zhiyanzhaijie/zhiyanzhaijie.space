use crate::models::post::get_all_posts;
use crate::models::tag::Tag;
use crate::components::providers::preference_provider::{resolve_locale, PreferenceContext};
use crate::root::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn TagListView() -> Element {
    let preference = use_context::<PreferenceContext>();
    let current_lang = resolve_locale(preference.read().locale.as_deref());

    let tag_posts = use_memo(move || {
        let current_lang = resolve_locale(preference.read().locale.as_deref());

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

    rsx! {
        div {
            class: "max-w-2xl mx-auto space-y-8",

            div {
                h1 {
                    class: "text-xl sm:text-2xl font-semibold tracking-tight text-foreground mb-2",
                    "Tag"
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    "{sorted_tags.read().len()} tags, {get_all_posts().iter().filter(|(meta, _)| meta.lang == current_lang).count()} articles"
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
                        class: "space-y-3",

                        Link {
                            class: "flex items-center space-x-2 sm:space-x-3",
                            to: Route::BlogByTag { tag: tag.to_string() },
                            h2 {
                                class: "text-sm sm:text-base font-medium text-foreground hover:underline underline-offset-4",
                                "#{ tag.label_en() }"
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
