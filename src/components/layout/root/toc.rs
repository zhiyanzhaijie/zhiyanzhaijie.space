use crate::components::icons::MenuIcon;
use crate::components::providers::interactive_provider::InteractiveContext;
use crate::utils::markdown_toc::TocItem;
use dioxus::prelude::*;

#[component]
fn RootContentTocInner(toc_items: Vec<TocItem>) -> Element {
    let interactive_context = use_context::<InteractiveContext>();
    let is_post_focused = (interactive_context.post_focus)();
    rsx! {
        aside {
            class: "w-52 max-h-full mt-2 mb-2 ml-auto pr-1 text-right flex flex-col",
            div {
                class: "shrink-0 pb-2 flex justify-end text-muted-foreground opacity-65",
                div { class: "scale-[70%]", MenuIcon {} }
            }
            if is_post_focused {
                nav {
                    class: "space-y-1 overflow-y-auto min-h-0",
                    for item in toc_items.iter() {
                        a {
                            href: "#{item.id}",
                            class: format!(
                                "block text-xs leading-5 transition-colors transition-opacity opacity-65 hover:opacity-100 {}",
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
