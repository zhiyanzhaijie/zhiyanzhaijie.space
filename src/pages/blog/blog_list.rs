use crate::models::post::POSTS;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div {
            class: "space-y-1 w-[61.8%]",
            if POSTS.is_empty() {
                div {
                    class: "text-center py-12 text-muted-foreground",
                    "暂无文章"
                }
            } else {
                {POSTS.iter().map(|(post_meta, _post_content)| {
                    let slug_clone = post_meta.slug.clone();
                    let title_clone = post_meta.title.clone();
                    let date_clone = post_meta.date.clone();
                    let tags_clone = post_meta.tags.clone();

                    rsx! {
                        div {
                            key: "{slug_clone}",
                            class: "group flex items-center justify-between py-3 px-4 hover:bg-muted/30 transition-colors duration-150 rounded-sm",

                            div {
                                class: "flex items-center space-x-3 min-w-0 flex-1",

                                // 文章标题
                                Link {
                                    to: Route::BlogPost { slug: slug_clone.clone() },
                                    class: "text-sm font-medium text-foreground hover:text-primary transition-colors truncate",
                                    title: "{title_clone}",
                                    "{title_clone}"
                                }

                                // 标签
                                if let Some(tags) = &tags_clone {
                                    if let Some(first_tag) = tags.first() {
                                        Link {
                                            to: Route::TagList {},
                                            class: "flex-shrink-0 text-xs bg-secondary/60 text-secondary-foreground px-2 py-1 rounded-full hover:bg-secondary transition-colors",
                                            "{first_tag}"
                                        }
                                    }
                                }
                            }

                            // 右侧：日期和字数
                            div {
                                class: "flex items-center space-x-4 text-xs text-muted-foreground flex-shrink-0",
                                span {
                                    class: "font-mono",
                                    "{post_meta.word_count} 字"
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
