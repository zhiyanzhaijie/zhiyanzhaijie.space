use crate::models::post::get_all_posts;
use crate::routes::Route;
use crate::ACTIVE_LOCALE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn BlogList() -> Element {
    // Filter posts to only show current language
    let posts = use_memo(move || {
        let current_locale = *ACTIVE_LOCALE.read();
        let current_lang = current_locale.as_str();

        let all_posts = get_all_posts();

        // Only include posts that match the current language
        let mut filtered_posts: Vec<_> = all_posts
            .into_iter()
            .filter(|(meta, _)| meta.lang == current_lang)
            .collect();

        // Sort by date (newest first)
        filtered_posts.sort_by(|a, b| {
            use chrono::NaiveDate;
            let date_a = NaiveDate::parse_from_str(&a.0.date, "%Y-%m-%d")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            let date_b = NaiveDate::parse_from_str(&b.0.date, "%Y-%m-%d")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            date_b.cmp(&date_a)
        });

        filtered_posts
    });

    rsx! {
        div {
            class: "space-y-1 w-full max-w-4xl",
            if posts().is_empty() {
                div {
                    class: "text-center py-12 text-muted-foreground",
                    "No articles available"
                }
            } else {
                {posts().iter().map(|(post_meta, _post_content)| {
                    let slug_clone = post_meta.slug.clone();
                    let title_clone = post_meta.title.clone();
                    let date_clone = post_meta.date.clone();
                    let tags_clone = post_meta.tags.clone();

                    rsx! {
                        div {
                            key: "{slug_clone}",
                            class: "group flex flex-col sm:flex-row sm:items-center justify-between py-3 px-3 sm:px-4 hover:bg-muted/30 transition-colors duration-150 rounded-sm gap-2 sm:gap-0",

                            div {
                                class: "flex items-center space-x-2 sm:space-x-3 min-w-0 flex-1 flex-wrap gap-y-1 sm:gap-y-0",

                                // 文章标题
                                Link {
                                    to: Route::BlogPost { slug: slug_clone.clone() },
                                    class: "text-sm sm:text-sm font-medium text-foreground hover:text-primary transition-colors truncate order-1 flex-shrink min-w-0",
                                    title: "{title_clone}",
                                    "{title_clone}"
                                }

                                // 标签
                                if let Some(tags) = &tags_clone {
                                    if let Some(first_tag) = tags.first() {
                                        Link {
                                            to: Route::BlogByTag { tag: first_tag.to_string() },
                                            class: "flex-shrink-0 text-xs bg-secondary/60 text-secondary-foreground px-2 py-1 rounded-full hover:bg-secondary transition-colors order-2 sm:order-2",
                                            { t!(first_tag.i18n_key()) }
                                        }
                                    }
                                }
                            }

                            // 右侧：日期和字数
                            div {
                                class: "flex items-center justify-start sm:justify-end space-x-3 sm:space-x-4 text-xs text-muted-foreground flex-shrink-0",
                                span {
                                    class: "font-mono hidden sm:inline",
                                    { t!("page-blog-post-word-count", count: post_meta.word_count) }
                                }
                                span {
                                    class: "font-mono text-xs",
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
