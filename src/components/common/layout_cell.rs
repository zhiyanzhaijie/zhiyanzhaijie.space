use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayoutCellPadding {
    Tight,
    Normal,
    Loose,
}

impl LayoutCellPadding {
    fn class_name(self) -> &'static str {
        match self {
            Self::Tight => "py-4 sm:py-6",
            Self::Normal => "py-6 sm:py-8",
            Self::Loose => "py-8 sm:py-10",
        }
    }
}

#[component]
pub fn LayoutCell(padding: LayoutCellPadding, children: Element) -> Element {
    let class_name = padding.class_name();
    rsx! {
        div {
            class: "{class_name}",
            {children}
        }
    }
}
