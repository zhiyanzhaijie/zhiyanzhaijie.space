use crate::components::views::blog::by_tag::BlogByTagView;
use dioxus::prelude::*;

#[component]
pub fn BlogByTag(tag: String) -> Element {
    rsx! { BlogByTagView { tag: tag } }
}
