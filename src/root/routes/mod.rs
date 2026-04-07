mod about;
mod blog_by_tag;
mod blog_list;
mod blog_post;
mod not_found;
mod tags;

use super::layouts::{BlogLayout, RootLayout};
use about::About;
use blog_by_tag::BlogByTag;
use blog_list::BlogList;
use blog_post::BlogPost;
use dioxus::prelude::*;
use not_found::PageNotFound;
use tags::TagList;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[redirect("/", || Route::BlogList {})]

        #[route("/about")]
        About {},

        #[nest("/tags")]
          #[route("/")]
          TagList {},
          #[route("/:tag")]
          BlogByTag { tag: String },
        #[end_nest]

        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                BlogList {},
                #[route("/post/:slug")]
                BlogPost { slug: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
