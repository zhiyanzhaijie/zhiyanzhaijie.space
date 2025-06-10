use dioxus::prelude::*;

use crate::models::post::POSTS;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    let posts_ref = POSTS
        .iter()
        .map(|(meta, _)| meta.clone())
        .collect::<Vec<_>>();
    
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold text-foreground mb-6", "博客文章" }
            if posts_ref.is_empty() {
                p { class: "text-muted-foreground", "暂时没有文章。" }
            } else {
                ul { class: "space-y-6",
                    for post_meta in posts_ref {
                        li { class: "p-4 bg-card rounded-lg shadow-sm border border-border",
                            Link {
                                class: "no-underline",
                                to: Route::BlogPost { slug: post_meta.slug.clone() },
                                h2 { class: "text-xl font-semibold text-primary mb-2", {post_meta.title.clone()} }
                            }
                            p { class: "text-sm text-muted-foreground", {post_meta.date.clone()} }
                        }
                    }
                }
            }
        }
    }
}