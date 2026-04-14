use dioxus::prelude::*;
use dioxus_markdown::CustomComponents;
use dioxus_use_js::use_js;
use std::sync::atomic::{AtomicUsize, Ordering};

use_js!("src/js/mermaid_bridge.js"::js_render_mermaid_by_base_id);

static MERMAID_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn next_mermaid_base_id() -> String {
    format!(
        "md-mermaid-{}",
        MERMAID_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}

#[component]
pub fn MermaidComponent(content: Option<String>, children: Element) -> Element {
    let base_id = use_hook(next_mermaid_base_id);
    let source_id = format!("{base_id}-src");
    let output_id = format!("{base_id}-out");

    use_effect(move || {
        let base_id = base_id.clone();
        spawn(async move {
            let _ = js_render_mermaid_by_base_id::<()>(base_id).await;
        });
    });

    rsx! {
        div { class: "my-4 mermaid-block",
            div {
                id: "{source_id}",
                style: "display:none;",
                if let Some(content) = content {
                    "{content}"
                } else {
                    {children}
                }
            }
            div { id: "{output_id}", class: "mermaid-output" }
        }
    }
}

pub fn registe_md_comp(components: &mut CustomComponents) {
    components.register("Mermaid", |props| {
        let content = props.get("content");
        let children = props.children;
        Ok(rsx! {
            MermaidComponent { content, children }
        })
    });
}
