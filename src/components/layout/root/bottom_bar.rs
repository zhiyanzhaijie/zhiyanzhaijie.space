use crate::{
    components::common::{
        breadcrumb::CommonBreadcrumb, locale_switcher::LocaleSwitcher, theme_switcher::ThemeSwitcher,
    },
};
use dioxus::prelude::*;

#[component]
pub fn RootBottomBar() -> Element {
    rsx! {
        div {
            class: "fixed bottom-0 left-0 right-0 z-20 px-6 pointer-events-none",
            div {
                class: "w-full min-w-0 max-w-[65ch] mx-auto py-2 flex items-center justify-between gap-2 pointer-events-auto",
                CommonBreadcrumb { compact: true, class: "min-w-0 flex-1 text-muted-foreground" }
                div {
                    class: "flex items-center gap-1 shrink-0",
                    ThemeSwitcher { is_mobile: true, compact: true }
                    LocaleSwitcher { compact: true }
                }
            }
        }
    }
}
