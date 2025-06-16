use dioxus::prelude::*;

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
                    nav { class: "mb-6 py-4",
                        Link {
                            class: "text-primary hover:text-primary/80 flex items-center",
                              to: Route::BlogList { },
                            "← 返回首页"
                        }
                    }
                    article { class: "container mx-auto",
                        h1 { class: "text-3xl font-bold text-foreground mb-2", {meta.title.clone()} }
                        p { class: "text-sm text-muted-foreground mb-8", {meta.date.clone()} }

                        div { class: "bg-muted-foreground w-full h-[1px] mb-8"}

                        MarkdownRenderer { content: content.clone() }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "container mx-auto px-4 py-16 text-center",
                    h1 { class: "text-3xl font-bold text-foreground mb-4", "找不到文章" }
                    p { class: "text-muted-foreground mb-6", "无法找到指定的文章。" }
                    Link {
                        class: "btn text-primary-foreground bg-primary px-4 py-2 rounded-md inline-block",
                        to: Route::BlogList { },
                        "返回首页"
                    }
                }
            }
        }
    }
}
