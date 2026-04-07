use crate::models::post::get_all_posts;
use crate::models::tag::Tag;
use crate::root::{Route, ACTIVE_LOCALE};
use dioxus::prelude::*;
use std::str::FromStr;

#[component]
pub fn BlogByTagView(tag: String) -> Element {
    let current_locale = *ACTIVE_LOCALE.read();
    let current_lang = current_locale.as_str();

    let posts = get_all_posts();
    let tag_enum = match Tag::from_str(&tag) {
        Ok(t) => t,
        Err(_) => {
            return rsx! {
                div {
                    class: "text-center py-8 sm:py-12 text-muted-foreground space-y-3 max-w-2xl mx-auto",
                    div {
                        class: "text-sm sm:text-base",
                        "Sorry, we couldn't find the page you're looking for."
                    }
                    Link {
                        to: Route::TagList {},
                        class: "inline-flex items-center text-sm text-primary hover:text-primary/80 transition-colors min-h-[44px] justify-center",
                        "Back to Home"
                    }
                }
            };
        }
    };

    let filtered_posts: Vec<_> = posts
        .iter()
        .filter(|(post_meta, _)| {
            post_meta.lang == current_lang
                && post_meta
                    .tags
                    .as_ref()
                    .map(|tags| tags.contains(&tag_enum))
                    .unwrap_or(false)
        })
        .collect();

    let mut sorted_posts = filtered_posts;
    sorted_posts.sort_by(|(a, _), (b, _)| b.date.cmp(&a.date));

    rsx! {
        div {
            class: "space-y-1 max-w-2xl mx-auto",

            div {
                class: "mb-5",

                nav {
                    class: "mb-2 sm:mb-3 text-xs sm:text-sm text-muted-foreground overflow-x-auto",
                    span { class: "mx-1 sm:mx-2", "/" }
                    Link {
                        to: Route::TagList {},
                        class: "hover:text-foreground transition-colors whitespace-nowrap",
                        "Tag"
                    }
                    span { class: "mx-1 sm:mx-2", "/" }
                    span { class: "text-foreground whitespace-nowrap", "{tag_enum.label_en()}" }
                }

                h1 {
                    class: "text-xl sm:text-2xl font-semibold tracking-tight text-foreground mb-2 leading-tight",
                    "Articles with \"{tag_enum.label_en()}\""
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    "Related to {tag_enum.label_en()}"
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
                {sorted_posts.into_iter().map(|(post_meta, _post_content)| {
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
                                            .filter(|t| *t != &tag_enum)
                                            .take(2)
                                            .map(|other_tag| rsx! {
                                                Link {
                                                    key: "{other_tag}",
                                                    to: Route::BlogByTag { tag: other_tag.to_string() },
                                                    class: "flex-shrink-0 text-xs text-muted-foreground hover:text-foreground transition-colors",
                                                    title: "View tag",
                                                    "#{ other_tag.label_en() }"
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
