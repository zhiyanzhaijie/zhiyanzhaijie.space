use crate::components::views::blog::list::BlogListView;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    rsx! { BlogListView {} }
}
