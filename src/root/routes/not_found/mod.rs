use crate::components::views::not_found::NotFoundView;
use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! { NotFoundView { route: route } }
}
