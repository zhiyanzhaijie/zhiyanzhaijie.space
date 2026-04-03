use crate::components::views::tags::TagListView;
use dioxus::prelude::*;

#[component]
pub fn TagList() -> Element {
    rsx! { TagListView {} }
}
