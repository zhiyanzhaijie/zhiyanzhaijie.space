use crate::root::{AppTheme, ACTIVE_THEME};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};

#[component]
pub fn MarkdownRenderer(content: String, components: ReadSignal<CustomComponents>) -> Element {
    let theme = match *ACTIVE_THEME.read() {
        AppTheme::Dark => Some("base16-ocean.dark"),
        AppTheme::Light => Some("base16-ocean.light"),
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
