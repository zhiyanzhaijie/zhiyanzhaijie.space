use crate::models::post::get_all_posts;
use crate::models::tag::Tag;
use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::str::FromStr;

#[component]
pub fn BlogByTag(tag: String) -> Element {
    let posts = get_all_posts();
    // Parse the tag string to Tag enum
    let tag_enum = match Tag::from_str(&tag) {
        Ok(t) => t,
        Err(_) => {
            // If tag doesn't exist, show error message
            return rsx! {
                div {
                    class: "text-center py-8 sm:py-12 text-muted-foreground space-y-3 max-w-2xl mx-auto",
                    div {
                        class: "text-sm sm:text-base",
                        { t!("page-not-found-message") }
                    }
                    Link {
                        to: Route::TagList {},
                        class: "inline-flex items-center text-sm text-primary hover:text-primary/80 transition-colors min-h-[44px] justify-center",
                        { t!("page-not-found-back-home") }
                    }
                }
            };
        }
    };

    // 过滤指定标签的文章
    let filtered_posts: Vec<_> = posts
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
            class: "space-y-1 max-w-4xl mx-auto",

            // 标签标题和面包屑导航
            div {
                class: "mb-4 sm:mb-6 pb-3 sm:pb-4 border-b border-border/30",

                // 面包屑导航
                nav {
                    class: "mb-2 sm:mb-3 text-xs sm:text-sm text-muted-foreground overflow-x-auto",
                    span { class: "mx-1 sm:mx-2", "/" }
                    Link {
                        to: Route::TagList {},
                        class: "hover:text-foreground transition-colors whitespace-nowrap",
                        { t!("page-tag-list-title") }
                    }
                    span { class: "mx-1 sm:mx-2", "/" }
                    span { class: "text-foreground whitespace-nowrap", { t!(tag_enum.i18n_key()) } }
                }

                h1 {
                    class: "text-lg sm:text-xl font-medium text-foreground mb-2 leading-tight",
                    { t!("page-blog-by-tag-title", tagName: t!(tag_enum.i18n_key())) }
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    { t!("page-blog-by-tag-description", tagName: t!(tag_enum.i18n_key())) }
                }
            }

            // 文章列表 - 极简设计
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
                            class: "group flex flex-col sm:flex-row sm:items-center justify-between py-3 px-3 sm:px-4 hover:bg-muted/30 transition-colors duration-150 rounded-sm gap-2 sm:gap-0",

                            // 左侧：标题和其他标签
                            div {
                                class: "flex items-center space-x-2 sm:space-x-3 min-w-0 flex-1 flex-wrap gap-y-1 sm:gap-y-0",

                                // 文章标题
                                Link {
                                    to: Route::BlogPost { slug: slug_clone.clone() },
                                    class: "text-sm font-medium text-foreground hover:text-primary transition-colors truncate order-1 flex-shrink min-w-0",
                                    title: "{title_clone}",
                                    "{title_clone}"
                                }

                                // 显示其他标签（最多2个）
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
                                class: "flex items-center justify-start sm:justify-end space-x-3 sm:space-x-4 text-xs text-muted-foreground flex-shrink-0",
                                span {
                                    class: "font-mono hidden sm:inline",
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
                    class: "pt-4 sm:pt-6 mt-4 sm:mt-6 border-t border-border/30 text-center",
                    Link {
                        to: Route::TagList {},
                        class: "inline-flex items-center text-sm text-muted-foreground hover:text-foreground transition-colors min-h-[44px] justify-center",
                        "← "
                        { t!("page-tag-list-title") }
                    }
                }
            }
        }
    }
}
