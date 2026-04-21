use crate::components::icons::MenuIcon;
use crate::components::providers::interactive_provider::InteractiveContext;
use crate::utils::markdown_toc::TocItem;
use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/toc_bridge.js"::js_bind_toc_scrollspy);

#[component]
fn RootContentTocInner(toc_items: Vec<TocItem>) -> Element {
    let interactive_context = use_context::<InteractiveContext>();
    let is_post_focused = (interactive_context.post_focus)();
    let heading_ids = toc_items
        .iter()
        .map(|item| item.id.clone())
        .collect::<Vec<_>>();

    use_effect(move || {
        if heading_ids.is_empty() {
            return;
        }
        let heading_ids = heading_ids.clone();
        spawn(async move {
            let _ = js_bind_toc_scrollspy::<()>("content-scroll-root".to_string(), heading_ids, 24)
                .await;
        });
    });
    rsx! {
        aside {
            class: "w-52 max-h-full mt-2 mb-2 pl-1 text-left flex flex-col",
            div {
                class: "shrink-0 pb-2 flex justify-start text-muted-foreground opacity-65",
                div { class: "scale-[70%]", MenuIcon {} }
            }
            if is_post_focused {
                nav {
                    class: "space-y-1 overflow-y-auto min-h-0",
                    for item in toc_items.iter() {
                        a {
                            href: "#{item.id}",
                            "data-toc-id": "{item.id}",
                            class: format!(
                                "block text-xs leading-5 transition-colors transition-opacity opacity-65 hover:opacity-100 {}",
                                if item.level >= 3 {
                                    "pl-3 text-muted-foreground hover:text-foreground"
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
