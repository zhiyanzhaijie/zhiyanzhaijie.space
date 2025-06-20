use crate::models::post::POSTS;
use crate::models::tag::Tag;
use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::collections::HashMap;

#[component]
pub fn TagList() -> Element {
    // 收集所有标签和对应的文章
    let tag_posts = use_memo(|| {
        let mut posts_map: HashMap<Tag, Vec<&crate::models::post::PostMetadata>> = HashMap::new();

        for (post_meta, _) in POSTS.iter() {
            if let Some(tags) = &post_meta.tags {
                for tag in tags {
                    posts_map
                        .entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(post_meta);
                }
            }
        }
        posts_map
    });

    // 按标签名排序
    let sorted_tags = use_memo(move || {
        let mut tags: Vec<Tag> = tag_posts.read().keys().cloned().collect();
        tags.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        tags
    });

    // 用于追踪当前活跃的标签section - 使用点击而非滚动检测
    let mut active_tag = use_signal(|| None::<Tag>);

    rsx! {
        div {
            class: "flex gap-8",

            // 主要内容区域
            div {
                class: "flex-1 space-y-8",

                // 页面标题
                div {
                    class: "mb-8 pb-6 border-b border-border/30",
                    h1 {
                        class: "text-lg font-medium text-foreground mb-2",
                        { t!("page-tag-list-title") }
                    }
                    p {
                        class: "text-sm text-muted-foreground",
                        { t!("page-tag-list-stats",
                             tagCount: sorted_tags.read().len(),
                             postCount: POSTS.len()) }
                    }
                }

                // 标签分组
                {sorted_tags.read().iter().enumerate().map(|(index, tag)| {
                    let binding = tag_posts.read();
                    let posts = binding.get(tag).unwrap();
                    let mut sorted_posts = posts.clone();
                    // 按日期排序（最新的在前）
                    sorted_posts.sort_by(|a, b| b.date.cmp(&a.date));

                    rsx! {
                        section {
                            key: "{tag}",
                            id: "tag-{index}",
                            class: "space-y-4",

                            // 标签标题
                            Link {
                                class: "flex items-center space-x-3 pb-3 border-b border-border/20",
                                to: Route::BlogByTag { tag: tag.to_string() },
                                h2 {
                                    class: "text-base font-medium text-foreground hover:underline",
                                    "#{ t!(tag.i18n_key()) }"
                                }
                                span {
                                    class: "flex-shrink-0 text-xs bg-secondary/60 text-secondary-foreground px-2 py-1 rounded-full hover:bg-secondary transition-colors",
                                    "{sorted_posts.len()}"
                                }
                            }

                            // 该标签下的文章列表
                            div {
                                class: "space-y-1 ml-4",
                                {sorted_posts.iter().map(|post_meta| {
                                    let slug_clone = post_meta.slug.clone();
                                    let title_clone = post_meta.title.clone();
                                    let date_clone = post_meta.date.clone();

                                    rsx! {
                                        div {
                                            key: "{slug_clone}",
                                            class: "group flex items-center justify-between py-2 px-3 hover:bg-muted/30 transition-colors duration-150 rounded-sm",

                                            // 文章标题
                                            Link {
                                                to: Route::BlogPost { slug: slug_clone.clone() },
                                                class: "text-sm font-medium text-foreground hover:text-primary transition-colors truncate flex-1",
                                                title: "{title_clone}",
                                                "{title_clone}"
                                            }

                                            // 日期和字数
                                            div {
                                                class: "flex items-center space-x-3 text-xs text-muted-foreground flex-shrink-0 ml-4",
                                                span {
                                                    class: "font-mono",
                                                    { t!("page-blog-post-word-count", count: post_meta.word_count) }
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

            if !sorted_tags.is_empty() {
                nav {
                    class: "w-48 flex-shrink-0 sticky top-36 self-start",

                    // 添加导航标题
                    div {
                        class: "mb-4 text-xs font-medium text-muted-foreground uppercase tracking-wide text-right",
                        "Quick Navigation"
                    }

                    div {
                        class: "space-y-2 max-h-[calc(100vh-200px)] overflow-y-auto overflow-x-hidden",
                        {sorted_tags.read().iter().enumerate().map(|(index, tag)| {
                            // 为每个标签生成不同的颜色
                            let hue = (index as f32 * 137.5) % 360.0; // 黄金角度分布
                            let binding = tag_posts.read();
                            let post_count = binding.get(tag).unwrap().len();
                            let is_active = active_tag.read().as_ref() == Some(tag);
                            let tag_clone = tag.clone();

                            // 统一的颜色算法
                            let color_style = format!("background-color: hsl({}, 65%, 60%)", hue as i32);

                            rsx! {
                                a {
                                    key: "{tag}",
                                    href: "#tag-{index}",
                                    onclick: move |_| {
                                        active_tag.set(Some(tag_clone.clone()));
                                    },
                                    class: "block w-full cursor-pointer group",

                                    div {
                                        class: "relative flex items-center h-12 overflow-hidden transition-colors duration-300",

                                        // 渐变背景层 - active时显示
                                        if is_active {
                                            div {
                                                class: "absolute -z-1 inset-0 transition-opacity duration-300 bg-[linear-gradient(to_left,color-mix(in_oklch,var(--color-ring),transparent_88%)_0%,transparent_62%)]",
                                            }
                                        }

                                        // 文字内容区域 - 使用flex布局自动贴近右侧色块
                                        div {
                                            class: "flex-1 flex flex-col items-end pr-3 transition-all duration-300",

                                            // 标签名称
                                            div {
                                                class: format!(
                                                    "font-semibold transition-colors duration-300 truncate max-w-28 text-right text-muted-foreground {}",
                                                    if is_active {
                                                        "text-sm"
                                                    } else {
                                                        "text-xs group-hover:opacity-80"
                                                    }
                                                ),
                                                "{ t!(tag.i18n_key()) }"
                                            }

                                            // 文章数量
                                            div {
                                                class: format!(
                                                    "font-medium transition-colors duration-300 {}",
                                                    if is_active {
                                                        "text-muted-foreground text-xs"
                                                    } else {
                                                        "text-muted-foreground/70 text-xs group-hover:text-muted-foreground"
                                                    }
                                                ),
                                                "{post_count}"
                                            }
                                        }

                                        // 右侧色块
                                        div {
                                            class: format!(
                                                "w-1 h-full transition-all duration-300 {}",
                                                if is_active {
                                                    "shadow-sm"
                                                } else {
                                                    "group-hover:shadow-sm"
                                                }
                                            ),
                                            style: "{color_style}",
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}
