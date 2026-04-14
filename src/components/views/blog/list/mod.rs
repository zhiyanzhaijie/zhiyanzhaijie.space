use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::providers::preference_provider::{
    resolve_locale, PreferenceContext, PreferenceStoreStoreExt,
};
use crate::components::views::blog::row::{BlogRow, BlogRowTagMode};
use crate::IO::blog;
use dioxus::prelude::*;

#[component]
pub fn BlogListView() -> Element {
    let preference = use_context::<PreferenceContext>();
    let locale = preference.locale();
    let posts_fut = use_server_future(move || {
        let current_lang = resolve_locale(locale.read().as_deref()).to_string();
        async move { blog::get_posts_by_lang(current_lang).await }
    })?;

    let posts = match posts_fut() {
        Some(Ok(posts)) => posts,
        _ => Vec::new(),
    };

    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "space-y-4",
                div {
                    h1 {
                        class: "text-sm sm:text-base font-semibold tracking-normal text-foreground leading-relaxed",
                        "Posts • {posts.len()}"
                    }
                }
                if posts.is_empty() {
                    div {
                        class: "text-center py-12 text-muted-foreground",
                        "No articles available"
                    }
                } else {
                    {posts.into_iter().map(|post_meta| {
                        let key = post_meta.slug.clone();
                        rsx! {
                            BlogRow {
                                key: "{key}",
                                post_meta: post_meta,
                                tag_mode: BlogRowTagMode::FirstTag,
                            }
                        }
                    })}
                }
            }
        }
    }
}
