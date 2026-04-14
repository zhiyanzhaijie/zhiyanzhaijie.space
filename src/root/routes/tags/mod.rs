use crate::components::views::tags::list::TagsListView;
use crate::components::views::tags::tag::TagsTagView;
use dioxus::prelude::*;

#[component]
pub fn TagList() -> Element {
    rsx! { TagsListView {} }
}

#[component]
pub fn TagsTag(tag: String) -> Element {
    rsx! { TagsTagView { tag: tag } }
}
