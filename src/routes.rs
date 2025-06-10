use dioxus::prelude::*;
use crate::pages::home::Home;
use crate::pages::blog_post::BlogPost;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:slug")]
    BlogPost { slug: String },
}

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        main {
            Outlet::<Route> {}
        }
    }
}