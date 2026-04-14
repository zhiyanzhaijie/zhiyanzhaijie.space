use dioxus::prelude::*;

#[component]
pub fn MoonIcon(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            ..attributes,
            path {
                fill: "none", // Moon icon is often an outline
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1",
                d: "M12 21a9 9 0 0 0 8.997-9.252a7 7 0 0 1-10.371-8.643A9 9 0 0 0 12 21"
            }
        }
    )
}
