use crate::root::Route;
use dioxus::prelude::*;

#[component]
pub fn RootLayoutContent(current_route: Route) -> Element {
    rsx! {
        section {
            class: "relative w-full min-w-0 text-muted-foreground",
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
    }
}
