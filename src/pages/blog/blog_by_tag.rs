use crate::models::post::POSTS;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogByTag(tag: String) -> Element {
    // 过滤指定标签的文章
    let filtered_posts: Vec<_> = POSTS
        .iter()
        .filter(|(post_meta, _)| {
            post_meta
                .tags
                .as_ref()
                .map(|tags| tags.contains(&tag))
                .unwrap_or(false)
        })
        .collect();

    rsx! {
        div {
            class: "space-y-1",

            // 标签标题
            div {
                class: "mb-6 pb-4 border-b border-border/30",
                h1 {
                    class: "text-lg font-medium text-foreground mb-2",
                    "#{tag}"
                }
                p {
                    class: "text-sm text-muted-foreground",
                    "{filtered_posts.len()} 篇文章"
                }
            }

            // 文章列表 - 极简设计
            if filtered_posts.is_empty() {
                div {
                    class: "text-center py-12 text-muted-foreground",
                    "该标签下暂无文章"
                }
            } else {
                {filtered_posts.into_iter().map(|(post_meta, _post_content)| {
                    let slug_clone = post_meta.slug.clone();
                    let title_clone = post_meta.title.clone();
                    let date_clone = post_meta.date.clone();
                    let tags_clone = post_meta.tags.clone();

                    rsx! {
                        div {
                            key: "{slug_clone}",
                            class: "group flex items-center justify-between py-3 px-4 hover:bg-muted/30 transition-colors duration-150 rounded-sm",

                            // 左侧：标题和标签
                            div {
                                class: "flex items-center space-x-3 min-w-0 flex-1",

                                // 文章标题
                                Link {
                                    to: Route::BlogPost { slug: slug_clone.clone() },
                                    class: "text-sm font-medium text-foreground hover:text-primary transition-colors truncate",
                                    title: "{title_clone}",
                                    "{title_clone}"
                                }

                                // 其他标签
                                if let Some(tags) = &tags_clone {
                                    {tags.iter().filter(|t| *t != &tag).take(1).map(|other_tag| rsx! {
                                        Link {
                                            key: "{other_tag}",
                                            to: Route::BlogByTag { tag: other_tag.clone() },
                                            class: "flex-shrink-0 text-xs bg-secondary/40 text-secondary-foreground px-2 py-1 rounded-full hover:bg-secondary/60 transition-colors",
                                            "{other_tag}"
                                        }
                                    })}
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
