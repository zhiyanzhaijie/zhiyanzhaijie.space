use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::providers::preference_provider::{
    resolve_locale, PreferenceContext, PreferenceStoreStoreExt,
};
use crate::root::Route;
use crate::IO::blog;
use dioxus::prelude::*;

#[component]
pub fn TagsListView() -> Element {
    let preference = use_context::<PreferenceContext>();
    let locale = preference.locale();
    let tag_groups_fut = use_server_future(move || {
        let current_lang = resolve_locale(locale.read().as_deref()).to_string();
        async move { blog::get_tag_groups(current_lang).await }
    })?;

    let tag_groups = match tag_groups_fut() {
        Some(Ok(groups)) => groups,
        _ => Vec::new(),
    };

    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "space-y-8",

                div {
                    h1 {
                        class: "text-sm sm:text-base font-semibold tracking-normal leading-relaxed mb-2",
                        "Tags • {tag_groups.len()}"
                    }
                }

                {tag_groups.iter().enumerate().map(|(index, group)| {
                    let tag = group.tag.clone();
                    let tag_id = tag.id.clone();
                    let tag_label = tag.label.clone();
                    let post_count = group.posts.len();

                    rsx! {
                        section {
                            key: "{tag}",
                            id: "tag-{index}",
                            class: "space-y-3",

                            Link {
                                class: "flex items-center space-x-2 sm:space-x-3",
                                to: Route::TagsTag { tag: tag_id.clone() },
                                h2 {
                                    class: "text-sm sm:text-base font-medium text-foreground hover:underline underline-offset-4 leading-relaxed",
                                    "#{ tag_label }"
                                }
                                span {
                                    class: "flex-shrink-0 text-sm text-muted-foreground",
                                    "{post_count}"
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}
