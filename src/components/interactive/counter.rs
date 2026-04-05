use dioxus::prelude::*;
use dioxus_markdown::CustomComponents;

#[component]
pub fn InteractiveCounterButton(label: String) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-foreground font-bold py-2 px-4 rounded",
            onclick: move |_| {
                count += 10;
            },
            {format!("{} (点击次数: {})", label, count())}
        }
    }
}

pub fn registe_md_comp(components: &mut CustomComponents) {
    components.register("IncrementCounter", |props| {
        let label = props.get("label").unwrap_or_else(|| "Counter".to_string());
        Ok(rsx! {
            InteractiveCounterButton { label }
        })
    });
}
