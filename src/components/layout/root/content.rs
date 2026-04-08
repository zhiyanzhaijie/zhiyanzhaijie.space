use crate::{
    components::layout::root::toc::RootContentToc, root::Route, utils::markdown_toc::TocItem,
};
use dioxus::prelude::*;

#[component]
pub fn RootLayoutContent(current_route: Route, toc_items: Vec<TocItem>) -> Element {
    let is_post_route = matches!(current_route, Route::BlogPost { .. });
    rsx! {
        section {
            class: "relative w-full min-w-0 text-muted-foreground",
            div {
                class: "w-full",
                div {
                    class: "w-full min-w-0 max-w-[65ch] mx-auto",
                    Outlet::<Route> {}
                }
            }
            if is_post_route && !toc_items.is_empty() {
                div {
                    class: "hidden lg:block fixed top-6 right-[max(1.5rem,calc((100vw-80rem)/2+1.5rem))] w-52 z-20",
                    RootContentToc { toc_items: toc_items.clone() }
                }
            }
        }
    }
}
