use crate::components::views::blog::list::BlogListView;
use crate::components::views::blog::post::BlogPostView;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    rsx! { BlogListView {} }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    rsx! { BlogPostView { slug: slug } }
}
