use dioxus::prelude::*;

#[component]
pub fn LinkedinIcon(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            ..attributes,
            circle {
                cx: "4",
                cy: "4",
                r: "2",
                fill: "currentColor",
                opacity: "0",
                animate {
                    fill: "freeze",
                    attribute_name: "opacity",
                    dur: "0.2s",
                    to: "1",
                }
            }
            g {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "4",
                g {
                    stroke_dasharray: "12",
                    stroke_dashoffset: "12",
                    path {
                        d: "M4 10v10",
                        animate {
                            fill: "freeze",
                            attribute_name: "stroke-dashoffset",
                            begin: "0.2s",
                            dur: "0.2s",
                            to: "0",
                        }
                    }
                    path {
                        d: "M10 10v10",
                        animate {
                            fill: "freeze",
                            attribute_name: "stroke-dashoffset",
                            begin: "0.5s",
                            dur: "0.2s",
                            to: "0",
                        }
                    }
                }
                path {
                    stroke_dasharray: "24",
                    stroke_dashoffset: "24",
                    d: "M10 15c0 -2.76 2.24 -5 5 -5c2.76 0 5 2.24 5 5v5",
                    animate {
                        fill: "freeze",
                        attribute_name: "stroke-dashoffset",
                        begin: "0.7s",
                        dur: "0.3s",
                        to: "0",
                    }
                }
            }
        }
    }
}
