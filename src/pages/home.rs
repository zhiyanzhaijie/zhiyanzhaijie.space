use dioxus::prelude::*;

use crate::models::post::POSTS;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold text-foreground mb-6 text-center", "博客文章" } // Reduced bottom margin

            if POSTS.is_empty() {
                p { class: "text-center text-muted-foreground", "暂时没有文章。" }
            } else {
                ul {
                    class: "space-y-1",
                    {POSTS.iter().map(|(post_meta, _post_content)| {
                        let slug_clone = post_meta.slug.clone();
                        let title_clone = post_meta.title.clone();
                        let date_clone = post_meta.date.clone();
                        let tags_clone = post_meta.tags.clone();

                        rsx! {
                            li {
                                key: "{slug_clone}",
                                class: "flex justify-between items-center py-2.5 px-3 border-b border-border hover:bg-muted/30 transition-colors duration-150 ease-in-out rounded-md", // Minimal padding, bottom border, subtle hover, slightly rounded

                                div {
                                    class: "flex items-center space-x-3 min-w-0 mr-4", // min-w-0 for truncate, mr-4 for spacing from right group

                                    Link {
                                        to: Route::BlogPost { slug: slug_clone.clone() },
                                        class: "text-sm font-medium text-primary hover:underline truncate block flex-shrink", // truncate and flex-shrink for title
                                        // Adding an explicit width or max-width can also help with truncation if parent doesn't constrain enough
                                        // e.g., "max-w-xs sm:max-w-sm md:max-w-md lg:max-w-lg"
                                        title: "{title_clone}", // HTML title attribute for full title on hover
                                        "{title_clone}"
                                    }

                                    if let Some(tags) = &tags_clone {
                                        if let Some(first_tag) = tags.first() {
                                            span {
                                                class: "text-xs bg-secondary text-secondary-foreground px-1.5 py-0.5 rounded-full whitespace-nowrap flex-shrink-0", // flex-shrink-0 to prevent tag from being truncated
                                                "{first_tag}"
                                            }
                                        }
                                    }
                                }

                                // Right group: Word count and date
                                div {
                                    class: "flex items-center space-x-3 text-xs text-muted-foreground whitespace-nowrap flex-shrink-0", // flex-shrink-0 to prevent this group from shrinking
                                    span { class: "font-medium", "{post_meta.word_count} 字" }
                                    span { "{date_clone}" }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
