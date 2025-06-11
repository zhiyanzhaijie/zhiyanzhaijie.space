use dioxus::prelude::*;

#[component]
pub fn TestHear() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-foreground font-bold py-2 px-4 rounded",
            onclick: move |_| {
                count += 10;
            },
            {format!("(点击次数: {})", count())}
        }
    }
}
