use crate::impls::blog::PostMetadata;
use crate::root::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BlogRowTagMode {
    FirstTag,
    ExcludingCurrentTag {
        tag_id: String,
        limit: usize,
    },
}

#[component]
pub fn BlogRow(post_meta: PostMetadata, tag_mode: BlogRowTagMode) -> Element {
    let slug = post_meta.slug.clone();
    let title = post_meta.title.clone();
    let date = post_meta.date.clone();
    let tags = post_meta.tags.clone().unwrap_or_default();
    let mut date_parts = date.splitn(3, '-');
    let date_year = date_parts.next().unwrap_or_default().to_string();
    let date_month = date_parts.next().unwrap_or_default().to_string();
    let date_day = date_parts.next().unwrap_or_default().to_string();

    rsx! {
        article {
            class: "group flex flex-col sm:flex-row sm:items-center justify-between py-2 gap-1 sm:gap-0",
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "flex flex-col min-w-0",
                    Link {
                        to: Route::BlogPost { slug: slug.clone() },
                        class: "text-base sm:text-lg tracking-tight text-foreground hover:underline underline-offset-4 transition-colors truncate min-w-0",
                        title: "{title}",
                        "{title}"
                    }
                    match tag_mode {
                        BlogRowTagMode::FirstTag => rsx! {
                            if let Some(first_tag) = tags.first() {
                                Link {
                                    to: Route::TagsTag { tag: first_tag.to_string() },
                                    class: "mt-0.5 inline-flex w-fit text-[11px] leading-none text-muted-foreground/75 hover:text-foreground transition-colors",
                                    "#{ first_tag.label }"
                                }
                            }
                        },
                        BlogRowTagMode::ExcludingCurrentTag { tag_id, limit } => rsx! {
                            div {
                                class: "mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-1",
                                {tags.iter()
                                    .filter(|tag| tag.id.as_str() != tag_id.as_str())
                                    .take(limit)
                                    .map(|other_tag| rsx! {
                                        Link {
                                            key: "{other_tag}",
                                            to: Route::TagsTag { tag: other_tag.id.clone() },
                                            class: "inline-flex text-[11px] leading-none text-muted-foreground/75 hover:text-foreground transition-colors",
                                            title: "View tag",
                                            "#{ other_tag.label }"
                                        }
                                    })
                                }
                            }
                        },
                    }
                }
            }

            div {
                class: "flex items-center justify-start sm:justify-end flex-shrink-0 font-mono text-sm",
                span { class: "text-muted-foreground opacity-45", "{date_year}" }
                span { class: "text-muted-foreground opacity-60", "-" }
                span { class: "text-muted-foreground opacity-80", "{date_month}" }
                span { class: "text-muted-foreground opacity-80", "-" }
                span {
                    class: "text-muted-foreground opacity-80",
                    title: "Published date",
                    "{date_day}"
                }
            }
        }
    }
}
