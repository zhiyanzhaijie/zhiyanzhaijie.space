use crate::root::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogLayout() -> Element {
    rsx! {
        div {
            class: "h-full w-full",
            Outlet::<Route> {}
        }
    }
}
