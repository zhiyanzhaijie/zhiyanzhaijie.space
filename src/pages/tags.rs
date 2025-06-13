use crate::models::post::POSTS;
use crate::routes::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn TagList() -> Element {
    // 收集所有标签和对应的文章
    let mut tag_posts: HashMap<String, Vec<&crate::models::post::PostMetadata>> = HashMap::new();

    for (post_meta, _) in POSTS.iter() {
        if let Some(tags) = &post_meta.tags {
            for tag in tags {
                tag_posts
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(post_meta);
            }
        }
    }

    // 按标签名排序
    let mut sorted_tags: Vec<_> = tag_posts.keys().collect();
    sorted_tags.sort();

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
                        "{sorted_tags.len()} 个标签，{POSTS.len()} 篇文章"
                    }
                }

                // 标签分组
                {sorted_tags.iter().enumerate().map(|(index, tag)| {
                    let posts = tag_posts.get(*tag).unwrap();
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
                                                to: Route::BlogPost { slug: slug_clone },
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

            // 右侧导航条
            if !sorted_tags.is_empty() {
                nav {
                    class: "w-16 flex-shrink-0 sticky top-24 self-start",
                    div {
                        class: "space-y-2",
                        {sorted_tags.iter().enumerate().map(|(index, tag)| {
                            // 为每个标签生成不同的颜色
                            let hue = (index as f32 * 137.5) % 360.0; // 黄金角度分布
                            let color_style = format!("background-color: hsl({}, 60%, 70%)", hue as i32);

                            rsx! {
                                a {
                                    key: "{tag}",
                                    href: "#tag-{index}",
                                    class: "block w-4 h-8 rounded-sm transition-all duration-200 hover:w-6 hover:shadow-sm",
                                    style: "{color_style}",
                                    title: "{tag} ({tag_posts.get(*tag).unwrap().len()} 篇文章)",
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}
