use dioxus::prelude::*;

#[component]
pub fn ColorPickerComponent(initial_color: String) -> Element {
    let mut color = use_signal(|| initial_color.clone());
    let mut show_picker = use_signal(|| false);

    rsx! {
        div {
            div {
                style: format!("width: 50px; height: 50px; cursor: pointer; border: 1px solid black; background-color: {}", color()),
                onclick: move |_| {
                    show_picker.set(!show_picker());
                }
            }

            {
                if show_picker() {
                    rsx! {
                        div {
                            class: "color-picker",
                            div {
                                class: "color-options",
                                for preset_color in ["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff"].iter() {
                                    div {
                                        style: format!("width: 25px; height: 25px; display: inline-block; margin: 2px; cursor: pointer; background-color: {}", preset_color),
                                        onclick: move |_| {
                                            color.set(preset_color.to_string());
                                            show_picker.set(false);
                                        }
                                    }
                                }
                            }
                            input {
                                r#type: "text",
                                value: color().to_string(),
                                oninput: move |evt| {
                                    color.set(evt.value().clone());
                                }
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}