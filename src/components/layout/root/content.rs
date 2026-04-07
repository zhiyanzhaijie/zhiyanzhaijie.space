use crate::{
    components::layout::root::toc::RootContentToc, root::Route, utils::markdown_toc::TocItem,
};
use dioxus::prelude::*;

#[component]
pub fn RootLayoutContent(current_route: Route, toc_items: Vec<TocItem>) -> Element {
    rsx! {
        section {
            class: "flex-1 min-w-0 overflow-x-hidden text-muted-foreground",
            if matches!(current_route, Route::BlogPost { .. }) {
                div {
                    class: "flex items-start gap-8",
                    div {
                        class: "flex-1 min-w-0 max-w-2xl",
                        Outlet::<Route> {}
                    }
                    if !toc_items.is_empty() {
                        RootContentToc { toc_items: toc_items.clone() }
                    }
                }
            } else {
                div {
                    class: "max-w-2xl",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
