use crate::{
    components::layout::root::{content::RootLayoutContent, sidebar::RootAsidebar},
    root::Route,
};
use dioxus::prelude::*;

#[component]
pub fn RootLayout() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        main {
            class: "fixed inset-0 w-screen h-screen overflow-x-hidden overflow-y-auto bg-background text-foreground font-sans min-h-0",
            div {
                class: "relative w-full min-h-0 max-w-7xl mx-auto px-6 flex flex-col gap-4",
                RootAsidebar { current_route: current_route.clone() }
                RootLayoutContent {
                    current_route: current_route,
                }
            }
        }
    }
}
