use crate::components::interactive::{code_runner, color_picker, counter, test};
use dioxus::prelude::*;
use dioxus_markdown::CustomComponents;

pub fn use_markdown_components() -> ReadSignal<CustomComponents> {
    use_hook(move || {
        let mut components = CustomComponents::new();
        counter::registe_md_comp(&mut components);
        test::registe_md_comp(&mut components);
        color_picker::registe_md_comp(&mut components);
        code_runner::registe_md_comp(&mut components);
        Signal::new(components).into()
    })
}
