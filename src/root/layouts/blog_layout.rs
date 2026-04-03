use crate::root::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogLayout() -> Element {
    rsx! {
        div {
            class: "w-full",
            Outlet::<Route> {}
        }
    }
}
