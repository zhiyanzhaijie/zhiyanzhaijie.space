use dioxus::prelude::*;

#[component]
pub fn I18NIcon(
    lang: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let (opacity_cn, opacity_en) = match lang.as_str() {
        "en" => ("0.5", "1"),
        _ => ("1", "0.5"),
    };

    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 512 512",
            ..attributes,
            path {
              fill: "none",
              opacity: opacity_cn,
              stroke: "currentColor",
              stroke_linecap: "round",
              stroke_linejoin: "round",
              stroke_width: "32",
              d: "M48 112h288 M281.3 112S257 206 199 277S80 384 80 384 M256 336s-35-27-72-75s-56-85-56-85 M192 64v48",
            }


            path {
              fill: "none",
              opacity: opacity_en,
              stroke: "currentColor",
              stroke_linecap: "round",
              stroke_linejoin: "round",
              stroke_width: "32",
              d: "m272 448l96-224l96 224m-162.5-64h133",
            }
        }
    )
}
