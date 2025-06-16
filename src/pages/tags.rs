use crate::models::post::POSTS;
use crate::routes::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn TagList() -> Element {
    // 收集所有标签和对应的文章
    let tag_posts = use_memo(|| {
        let mut posts_map: HashMap<String, Vec<&crate::models::post::PostMetadata>> =
            HashMap::new();

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
        let mut tags: Vec<String> = tag_posts.read().keys().cloned().collect();
        tags.sort();
        tags
    });

    // 用于追踪当前活跃的标签section - 使用点击而非滚动检测
    let mut active_tag = use_signal(|| None::<String>);

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
                        "标签"
                    }
                    p {
                        class: "text-sm text-muted-foreground",
                        "{sorted_tags.read().len()} 个标签，{POSTS.len()} 篇文章"
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
                            div {
                                class: "flex items-center space-x-3 pb-3 border-b border-border/20",
                                h2 {
                                    class: "text-base font-medium text-foreground",
                                    "#{tag}"
                                }
                                span {
                                    class: "text-xs text-muted-foreground bg-secondary/40 px-2 py-1 rounded-full",
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
                })}
            }

            if !sorted_tags.is_empty() {
                nav {
                    class: "w-48 flex-shrink-0 sticky top-24 self-start",

                    // 添加导航标题
                    div {
                        class: "mb-4 text-xs font-medium text-muted-foreground uppercase tracking-wide text-right",
                        "快速导航"
                    }

                    div {
                        class: "space-y-2 max-h-96 overflow-y-auto overflow-x-hidden",
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
                                        class: "relative flex items-center justify-end overflow-hidden",

                                        // 色块 - 从右侧开始，右侧无圆角
                                        div {
                                            class: format!(
                                                "relative h-8 transition-all duration-500 ease-in-out {}",
                                                if is_active {
                                                    "w-26 rounded-l-lg shadow-lg"
                                                } else {
                                                    "w-1 rounded-l-sm shadow-sm group-hover:w-2 group-hover:shadow-md"
                                                }
                                            ),
                                            style: "{color_style}",

                                            // 激活状态的文字（在色块内部）
                                            if is_active {
                                                div {
                                                    class: "absolute inset-0 flex items-center justify-center opacity-100 transition-opacity duration-300 delay-200",
                                                    div {
                                                        class: "text-right",
                                                        span {
                                                            class: "text-sm font-medium text-white drop-shadow-sm",
                                                            "{tag}"
                                                        }
                                                        span {
                                                            class: "text-xs text-white/90 ml-1",
                                                            "{post_count}"
                                                        }
                                                    }
                                                }
                                            }

                                            // 激活状态的微妙光效
                                            if is_active {
                                                div {
                                                    class: "absolute inset-0 bg-gradient-to-r from-white/0 via-white/10 to-white/0 rounded-l-lg"
                                                }
                                            }
                                        }

                                        // 非激活状态的文字（在色块外部右侧）
                                        if !is_active {
                                            div {
                                                class: "absolute right-3 flex flex-col items-end opacity-100 transition-all duration-300 group-hover:opacity-100",

                                                div {
                                                    class: "text-sm font-medium text-muted-foreground group-hover:text-foreground truncate max-w-32 text-right transition-colors duration-200",
                                                    "{tag}"
                                                }
                                                div {
                                                    class: "text-xs text-muted-foreground/70 group-hover:text-muted-foreground transition-colors duration-200",
                                                    "{post_count} 篇"
                                                }
                                            }
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
