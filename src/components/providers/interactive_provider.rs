use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct InteractiveContext {
    pub post_focus: Signal<bool>,
}

#[component]
pub fn InteractiveProvider(children: Element) -> Element {
    let post_focus = use_signal(|| false);
    let interactive_context = InteractiveContext { post_focus };
    use_context_provider(|| interactive_context);
    children
}
