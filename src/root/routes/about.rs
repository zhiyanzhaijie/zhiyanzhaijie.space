use crate::components::views::about::AboutView;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! { AboutView {} }
}
