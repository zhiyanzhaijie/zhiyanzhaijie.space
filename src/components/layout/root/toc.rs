use crate::utils::markdown_toc::TocItem;
use dioxus::prelude::*;

#[component]
fn RootContentTocInner(toc_items: Vec<TocItem>) -> Element {
    rsx! {
        aside {
            class: "w-full text-right",
            div {
                class: "space-y-2",
                div { class: "text-xs text-muted-foreground", "On This Page" }
                nav {
                    class: "space-y-1",
                    for item in toc_items.iter() {
                        a {
                            href: "#{item.id}",
                            class: format!(
                                "block text-xs transition-colors {}",
                                if item.level >= 3 {
                                    "pr-3 text-muted-foreground hover:text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            "{item.title}"
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn RootContentToc(toc_items: Vec<TocItem>) -> Element {
    let mut mounted = use_signal(|| false);

    use_effect(move || {
        mounted.set(true);
    });

    if !mounted() {
        return rsx! {};
    }

    rsx! {
        RootContentTocInner { toc_items }
    }
}
