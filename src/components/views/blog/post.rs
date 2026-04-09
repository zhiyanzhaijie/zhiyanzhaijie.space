use crate::components::common::layout_cell::{LayoutCell, LayoutCellPadding};
use crate::components::layout::root::toc::RootContentToc;
use crate::components::markdown::hooks::use_markdown_components;
use crate::components::markdown::renderer::MarkdownRenderer;
use crate::components::providers::interactive_provider::InteractiveContext;
use crate::components::providers::preference_provider::{resolve_locale, PreferenceContext};
use crate::root::Route;
use crate::utils::markdown_toc::inject_heading_anchors_and_collect_toc;
use crate::IO::blog;
use dioxus::document::Stylesheet;
use dioxus::prelude::*;
const MARKDOWN_CSS: Asset = asset!("/assets/markdown.css");

#[component]
pub fn BlogPostView(slug: String) -> Element {
    let markdown_components = use_markdown_components();
    let preference = use_context::<PreferenceContext>();
    let mut interactive_context = use_context::<InteractiveContext>();
    use_effect(move || {
        interactive_context.post_focus.set(false);
    });
    let post_fut = use_server_future(move || {
        let active_lang = resolve_locale(preference.read().locale.as_deref()).to_string();
        let slug = slug.clone();
        async move { blog::get_post_with_fallback(slug, active_lang).await }
    })?;

    match post_fut() {
        Some(Ok(Some(post))) => {
            let meta = post.meta;
            let content = post.content;
            let requested_locale = resolve_locale(preference.read().locale.as_deref()).to_string();
            let markdown_key = format!("{}-{requested_locale}", meta.slug);
            let (content_with_anchors, toc_items) =
                inject_heading_anchors_and_collect_toc(&content);
            rsx! {
                LayoutCell {
                    padding: LayoutCellPadding::Normal,
                    Stylesheet { href: MARKDOWN_CSS }
                    div {
                        onmouseenter: move |_| interactive_context.post_focus.set(true),
                        onmouseleave: move |_| interactive_context.post_focus.set(false),

                        article {
                            header { class: "mb-7",
                                h1 { class: "text-2xl sm:text-3xl font-semibold tracking-tight text-foreground mb-3 leading-tight", {meta.title.clone()} }


                                div { class: "flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4 text-sm text-muted-foreground",
                                    p { class: "flex items-center",
                                        "Published on {meta.date.clone()}"
                                    }

                                    span { class: "hidden sm:inline-block text-xs",
                                        "•"
                                    }
                                    span { class: "hidden sm:inline-block text-xs",
                                        "{meta.word_count} words"
                                    }
                                }

                                if let Some(ref tags) = meta.tags {
                                    div {
                                        class: "flex flex-wrap gap-2 mt-3 sm:mt-4",
                                        for tag in tags.iter() {
                                            Link {
                                                key: "{tag.id}",
                                                to: Route::BlogByTag { tag: tag.to_string() },
                                                class: "inline-flex items-center text-xs text-muted-foreground hover:text-foreground transition-colors",
                                                "#{ tag.label }"
                                            }
                                        }
                                    }
                                }
                            }

                            div {
                                key: "{markdown_key}",
                                class: "prose prose-sm sm:prose-base lg:prose-lg max-w-none prose-slate dark:prose-invert",
                                MarkdownRenderer {
                                    content: content_with_anchors.clone(),
                                    components: markdown_components,
                                }
                            }

                            nav { class: "mt-8 sm:mt-12 pt-6 sm:pt-8",
                                div { class: "hover:underline",
                                    Link {
                                        class: "inline-flex items-center gap-2 text-sm sm:text-base text-muted-foreground hover:text-foreground transition-colors",
                                        to: Route::BlogList { },
                                        "Back"
                                    }
                                }
                            }
                        }
                        if !toc_items.is_empty() {
                            div {
                                class: "hidden lg:block fixed top-0 bottom-0 left-[calc(50%+32.5ch)] right-0 z-20",
                                div {
                                    class: "h-full pt-6 pl-4 pr-6",
                                    RootContentToc { toc_items: toc_items.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            rsx! {
                LayoutCell {
                    padding: LayoutCellPadding::Normal,
                    div { class: "py-10 sm:py-16 text-center",
                        h1 { class: "text-xl sm:text-2xl lg:text-3xl font-semibold tracking-tight text-foreground mb-4 leading-tight", "Page Not Found" }
                        p { class: "text-sm sm:text-base text-muted-foreground mb-6 max-w-md mx-auto", "Sorry, we couldn't find the page you're looking for." }
                        Link {
                            class: "inline-flex items-center justify-center px-2 py-2 text-sm sm:text-base font-medium text-muted-foreground hover:text-foreground transition-colors min-h-[44px] underline-offset-4 hover:underline",
                            to: Route::BlogList { },
                            "Back to Home"
                        }
                    }
                }
            }
        }
    }
}
