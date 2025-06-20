use crate::models::post::POSTS;
use crate::models::tag::Tag;
use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::str::FromStr;

#[component]
pub fn BlogByTag(tag: String) -> Element {
    // Parse the tag string to Tag enum
    let tag_enum = match Tag::from_str(&tag) {
        Ok(t) => t,
        Err(_) => {
            // If tag doesn't exist, show error message
            return rsx! {
                div {
                    class: "text-center py-12 text-muted-foreground space-y-3",
                    div { { t!("page-not-found-message") } }
                    Link {
                        to: Route::TagList {},
                        class: "inline-flex items-center text-sm text-primary hover:text-primary/80 transition-colors",
                        { t!("page-not-found-back-home") }
                    }
                }
            };
        }
    };

    // 过滤指定标签的文章
    let filtered_posts: Vec<_> = POSTS
        .iter()
        .filter(|(post_meta, _)| {
            post_meta
                .tags
                .as_ref()
                .map(|tags| tags.contains(&tag_enum))
                .unwrap_or(false)
        })
        .collect();

    // 按日期排序（最新的在前）
    let mut sorted_posts = filtered_posts;
    sorted_posts.sort_by(|(a, _), (b, _)| b.date.cmp(&a.date));

    rsx! {
        div {
            class: "space-y-1",

            // 标签标题和面包屑导航
            div {
                class: "mb-6 pb-4 border-b border-border/30",

                // 面包屑导航
                nav {
                    class: "mb-3 text-sm text-muted-foreground",
                    span { class: "mx-2", "/" }
                    Link {
                        to: Route::TagList {},
                        class: "hover:text-foreground transition-colors",
                        { t!("page-tag-list-title") }
                    }
                    span { class: "mx-2", "/" }
                    span { class: "text-foreground", { t!(tag_enum.i18n_key()) } }
                }

                h1 {
                    class: "text-lg font-medium text-foreground mb-2",
                    { t!("page-blog-by-tag-title", tagName: t!(tag_enum.i18n_key())) }
                }
                p {
                    class: "text-sm text-muted-foreground",
                    { t!("page-blog-by-tag-description", tagName: t!(tag_enum.i18n_key())) }
                }
            }

            // 文章列表 - 极简设计
            if sorted_posts.is_empty() {
                div {
                    class: "text-center py-12 text-muted-foreground space-y-3",
                    div { "No articles found for this tag" }
                    Link {
                        to: Route::TagList {},
                        class: "inline-flex items-center text-sm text-primary hover:text-primary/80 transition-colors",
                        "← "
                        { t!("page-tag-list-title") }
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
                            class: "group flex items-center justify-between py-3 px-4 hover:bg-muted/30 transition-colors duration-150 rounded-sm",

                            // 左侧：标题和其他标签
                            div {
                                class: "flex items-center space-x-3 min-w-0 flex-1",

                                // 文章标题
                                Link {
                                    to: Route::BlogPost { slug: slug_clone.clone() },
                                    class: "text-sm font-medium text-foreground hover:text-primary transition-colors truncate",
                                    title: "{title_clone}",
                                    "{title_clone}"
                                }

                                // 显示其他标签（最多2个）
                                if let Some(tags) = &tags_clone {
                                    div {
                                        class: "flex items-center space-x-1 flex-shrink-0",
                                        {tags.iter()
                                            .filter(|t| *t != &tag_enum)
                                            .take(2)
                                            .map(|other_tag| rsx! {
                                                Link {
                                                    key: "{other_tag}",
                                                    to: Route::BlogByTag { tag: other_tag.to_string() },
                                                    class: "flex-shrink-0 text-xs bg-secondary/40 text-secondary-foreground px-2 py-1 rounded-full hover:bg-secondary/60 transition-colors",
                                                    title: "View tag",
                                                    { t!(other_tag.i18n_key()) }
                                                }
                                            })
                                        }
                                    }
                                }
                            }

                            // 右侧：日期和字数
                            div {
                                class: "flex items-center space-x-4 text-xs text-muted-foreground flex-shrink-0",
                                span {
                                    class: "font-mono",
                                    title: "Word count",
                                    { t!("page-blog-post-word-count", count: post_meta.word_count) }
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

                // 添加返回链接
                div {
                    class: "pt-6 mt-6 border-t border-border/30 text-center",
                    Link {
                        to: Route::TagList {},
                        class: "inline-flex items-center text-sm text-muted-foreground hover:text-foreground transition-colors",
                        "← "
                        { t!("page-tag-list-title") }
                    }
                }
            }
        }
    }
}
