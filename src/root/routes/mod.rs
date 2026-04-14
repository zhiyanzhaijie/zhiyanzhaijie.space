mod about;
mod blog;
mod not_found;
mod tags;

use super::layouts::{BlogLayout, RootLayout};
use about::About;
use blog::{BlogList, BlogPost};
use dioxus::prelude::*;
use not_found::PageNotFound;
use tags::{TagList, TagsTag};

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
          TagsTag { tag: String },
        #[end_nest]

        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                BlogList {},
                #[route("/:slug")]
                BlogPost { slug: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
