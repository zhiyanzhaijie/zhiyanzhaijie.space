use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::providers::preference_provider::{
    resolve_locale, PreferenceContext, PreferenceStoreStoreExt,
};
use crate::components::views::blog::row::{BlogRow, BlogRowTagMode};
use crate::root::Route;
use crate::IO::blog;
use dioxus::prelude::*;

#[component]
pub fn TagsTagView(tag: String) -> Element {
    let preference = use_context::<PreferenceContext>();
    let locale = preference.locale();
    let tag_id = tag.trim().to_lowercase();
    let query_tag_id = tag_id.clone();

    let posts_fut = use_server_future(move || {
        let current_lang = resolve_locale(locale.read().as_deref()).to_string();
        let tag = query_tag_id.clone();
        async move { blog::get_posts_by_tag(tag, current_lang).await }
    })?;

    let sorted_posts = match posts_fut() {
        Some(Ok(posts)) => posts,
        _ => Vec::new(),
    };
    let tag_label = sorted_posts
        .iter()
        .find_map(|post_meta| {
            post_meta
                .tags
                .as_ref()
                .and_then(|tags| tags.iter().find(|item| item.id.as_str() == tag_id.as_str()))
                .map(|item| item.label.clone())
        })
        .unwrap_or_else(|| tag_id.clone());

    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "space-y-1",

                div {
                    class: "mb-5",
                    h1 {
                        class: "text-sm sm:text-base font-semibold tracking-normal text-foreground mb-2 leading-relaxed",
                        "Posts of {tag_label} • {sorted_posts.len()}"
                    }
                }

                if sorted_posts.is_empty() {
                    div {
                        class: "text-center py-8 sm:py-12 text-muted-foreground space-y-3 max-w-md mx-auto",
                        div {
                            class: "text-sm sm:text-base leading-relaxed",
                            "No articles found for this tag"
                        }
                        Link {
                            to: Route::TagList {},
                            class: "inline-flex items-center text-sm sm:text-base text-primary hover:text-primary/80 transition-colors min-h-[44px] justify-center",
                            "← "
                            "Tag"
                        }
                    }
                } else {
                    {sorted_posts.into_iter().map(|post_meta| {
                        let key = post_meta.slug.clone();
                        rsx! {
                            BlogRow {
                                key: "{key}",
                                post_meta: post_meta,
                                tag_mode: BlogRowTagMode::ExcludingCurrentTag {
                                    tag_id: tag_id.clone(),
                                    limit: 2,
                                },
                            }
                        }
                    })}
                }
            }
        }
    }
}
