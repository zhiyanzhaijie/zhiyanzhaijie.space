use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogLayout() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto",
            // 直接渲染子路由内容，不添加额外装饰
            Outlet::<Route> {}
        }
    }
}
