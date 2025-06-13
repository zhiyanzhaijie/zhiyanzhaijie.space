use crate::components::layout::MainLayout;
use crate::pages::about::About;
use crate::pages::blog::BlogByTag;
use crate::pages::blog::BlogLayout;
use crate::pages::blog::BlogList;
use crate::pages::blog_post::BlogPost;
use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::tags::TagList;
use dioxus::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    // 主布局包装所有路由
    #[layout(MainLayout)]
        #[route("/")]
        Home {},

        #[route("/about")]
        About {},

        #[route("/tags")]
        TagList {},

        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                BlogList {},

                #[route("/post/:slug")]
                BlogPost { slug: String },

                #[route("/tag/:tag")]
                BlogByTag { tag: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]

    // 404页面
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
