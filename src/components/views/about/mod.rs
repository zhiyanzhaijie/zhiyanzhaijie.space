use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::markdown::hooks::use_markdown_components;
use crate::components::markdown::renderer::MarkdownRenderer;
use crate::components::providers::preference_provider::{
    resolve_locale, PreferenceContext, PreferenceStoreStoreExt,
};
use crate::IO::about;
use dioxus::prelude::*;

#[component]
pub fn AboutView() -> Element {
    let markdown_components = use_markdown_components();
    let preference = use_context::<PreferenceContext>();
    let locale = preference.locale();
    let markdown_fut = use_server_future(move || {
        let active_lang = resolve_locale(locale.read().as_deref()).to_string();
        async move { about::get_about_markdown(active_lang).await }
    })?;
    let markdown = match markdown_fut() {
        Some(Ok(value)) => value,
        _ => String::new(),
    };
    rsx! {
        LayoutCell {
            padding: LayoutCellPadding::Normal,
            div {
                class: "prose prose-sm sm:prose-base lg:prose-lg max-w-none prose-slate dark:prose-invert",
                MarkdownRenderer {
                    content: markdown,
                    components: markdown_components,
                }
            }
        }
    }
}
