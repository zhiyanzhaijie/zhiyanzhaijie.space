use crate::components::providers::preference_provider::{resolve_theme, PreferenceContext};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};

#[component]
pub fn MarkdownRenderer(content: String, components: ReadSignal<CustomComponents>) -> Element {
    let preference = use_context::<PreferenceContext>();
    let theme = match resolve_theme(preference.read().theme.as_deref()) {
        "dark" => Some("base16-ocean.dark"),
        _ => Some("base16-ocean.light"),
    };

    rsx! {
        div { class: "markdown-body",
            Markdown {
                src: content,
                theme: theme,
                components,
            }
        }
    }
}
