use crate::components::views::blog::post::BlogPostView;
use dioxus::prelude::*;

#[component]
pub fn BlogPost(slug: String) -> Element {
    rsx! { BlogPostView { slug: slug } }
}
