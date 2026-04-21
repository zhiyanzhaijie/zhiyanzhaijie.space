use crate::{
    components::layout::root::{bottom_bar::RootBottomBar, sidebar::RootAsidebar},
    root::Route,
};
use dioxus::prelude::*;

#[component]
pub fn RootLayout() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        main {
            class: "fixed inset-0 w-screen h-screen overflow-hidden overscroll-none text-foreground font-sans min-h-0",
            div {
                class: "relative w-full h-full min-h-0 mx-auto overflow-hidden flex flex-col",
                RootAsidebar { current_route: current_route.clone() }
                section {
                    id: "content-scroll-root",
                    class: "relative w-full min-w-0 text-muted-foreground flex-1 min-h-0 overflow-y-auto px-6 mb-8",
                    div {
                        class: "w-full",
                        div {
                            class: "w-full min-w-0 max-w-[65ch] mx-auto",
                            SuspenseBoundary {
                                fallback: |_| rsx! {},
                                Outlet::<Route> {}
                            }
                        }
                    }
                }
                RootBottomBar {}
            }
        }
    }
}
