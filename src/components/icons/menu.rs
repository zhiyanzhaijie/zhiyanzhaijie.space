use dioxus::prelude::*;

#[component]
pub fn MenuIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            class: "w-5 h-5 text-foreground",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                d: "M3 12h18",
            }
            path {
                d: "M3 6h18",
            }
            path {
                d: "M3 18h18",
            }
        }
    }
}
