use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::markdown::renderer::MarkdownRenderer;
use crate::models::post::get_post_by_slug;
use crate::routes::Route;

#[component]
pub fn BlogPost(slug: String) -> Element {
    let post = use_memo(move || get_post_by_slug(&slug));

    match post() {
        Some((meta, content)) => {
            rsx! {
                div { class: "w-full h-full px-4 py-4 overflow-hidden overflow-y-auto",

                    article { class: "container mx-auto",
                        h1 { class: "text-3xl font-bold text-foreground mb-2", {meta.title.clone()} }
                        p { class: "text-sm text-muted-foreground mb-2",
                            { t!("page-blog-post-published") } " {meta.date.clone()}"
                        }

                        // Tags display
                        if let Some(ref tags) = meta.tags {
                            div {
                                class: "mb-8",
                                for (i, tag) in tags.iter().enumerate() {
                                    if i > 0 {
                                        span { class: "text-muted-foreground", " " }
                                    }
                                    Link {
                                        to: Route::BlogByTag { tag: tag.to_string() },
                                        class: "text-muted-foreground hover:text-muted-foreground/80 hover:underline transition-colors",
                                        "#{ t!(tag.i18n_key()) }"
                                    }
                                }
                            }
                        }

                        MarkdownRenderer { content: content.clone() }
                    }

                    nav { class: "mt-6 py-4",
                        Link {
                            class: "text-primary hover:text-primary/80 flex items-center",
                              to: Route::BlogList { },
                            { t!("common-button-back") }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "container mx-auto px-4 py-16 text-center",
                    h1 { class: "text-3xl font-bold text-foreground mb-4", { t!("page-not-found-title") } }
                    p { class: "text-muted-foreground mb-6", { t!("page-not-found-message") } }
                    Link {
                        class: "btn text-primary-foreground bg-primary px-4 py-2 rounded-md inline-block",
                        to: Route::BlogList { },
                        { t!("page-not-found-back-home") }
                    }
                }
            }
        }
    }
}
