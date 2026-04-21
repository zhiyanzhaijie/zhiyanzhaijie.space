use crate::{
    components::icons::{MenuIcon, ZyzjIcon},
    root::Route,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn RootAsidebar(current_route: Route) -> Element {
    let mut is_mobile_nav_open = use_signal(|| false);
    let dim_opacity_class = "opacity-50";
    let is_articles_route = matches!(
        current_route,
        Route::BlogList { .. } | Route::BlogPost { .. }
    );
    let is_tags_route = matches!(current_route, Route::TagList { .. } | Route::TagsTag { .. });
    let is_about_route = matches!(current_route, Route::About { .. });
    rsx! {
        aside { class: "w-full px-6 md:px-0 md:w-[calc(50%-32.5ch)] md:fixed md:right-0 md:top-0 md:bottom-0 md:z-20",
            div { class: "py-2 md:h-full md:pl-2 md:pr-6 md:flex md:flex-col md:justify-end",
                div { class: "flex w-full items-center justify-between md:hidden",
                    Link {
                        to: Route::BlogList {},
                        class: "inline-flex items-center",
                        ZyzjIcon { class: format!("w-5 transition-opacity duration-200 {} hover:opacity-100", dim_opacity_class) }
                    }
                    div { class: "relative flex items-center gap-1",
                        button {
                            class: "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground hover:text-foreground transition-colors",
                            title: "Open menu",
                            onclick: move |_| is_mobile_nav_open.set(!is_mobile_nav_open()),
                            MenuIcon {}
                        }
                        if is_mobile_nav_open() {
                            nav { class: "absolute right-0 mt-3 w-52 space-y-2 rounded-md border border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-900 p-2 text-sm uppercase shadow-md backdrop-blur-md z-20",
                                Link {
                                    to: Route::BlogList {},
                                    class: format!(
                                        "block transition-colors {}",
                                        if is_articles_route {
                                            "text-foreground font-medium"
                                        } else {
                                            "text-muted-foreground hover:text-foreground"
                                        },
                                    ),
                                    onclick: move |_| is_mobile_nav_open.set(false),
                                    {t!("layout_root_asidebar_nav_articles")}
                                }
                                Link {
                                    to: Route::TagList {},
                                    class: format!(
                                        "block transition-colors {}",
                                        if is_tags_route {
                                            "text-foreground font-medium"
                                        } else {
                                            "text-muted-foreground hover:text-foreground"
                                        },
                                    ),
                                    onclick: move |_| is_mobile_nav_open.set(false),
                                    {t!("layout_root_asidebar_nav_tags")}
                                }
                                Link {
                                    to: Route::About {},
                                    class: format!(
                                        "block transition-colors {}",
                                        if is_about_route {
                                            "text-foreground font-medium"
                                        } else {
                                            "text-muted-foreground hover:text-foreground"
                                        },
                                    ),
                                    onclick: move |_| is_mobile_nav_open.set(false),
                                    {t!("layout_root_asidebar_nav_about")}
                                }
                            }
                        }
                    }
                }
                div { class: "hidden md:flex md:w-full md:justify-between",
                    nav { class: "self-end flex flex-col gap-1 text-left text-sm uppercase",
                        Link {
                            to: Route::BlogList {},
                            class: format!(
                                "block transition-colors transition-opacity duration-200 {} hover:text-foreground hover:opacity-100 {}",
                                dim_opacity_class,
                                if is_articles_route {
                                    "text-foreground font-medium"
                                } else {
                                    "text-muted-foreground"
                                },
                            ),
                            {t!("layout_root_asidebar_nav_articles")}
                        }
                        Link {
                            to: Route::TagList {},
                            class: format!(
                                "block transition-colors transition-opacity duration-200 {} hover:text-foreground hover:opacity-100 {}",
                                dim_opacity_class,
                                if is_tags_route {
                                    "text-foreground font-medium"
                                } else {
                                    "text-muted-foreground"
                                },
                            ),
                            {t!("layout_root_asidebar_nav_tags")}
                        }
                        Link {
                            to: Route::About {},
                            class: format!(
                                "block transition-colors transition-opacity duration-200 {} hover:text-foreground hover:opacity-100 {}",
                                dim_opacity_class,
                                if is_about_route {
                                    "text-foreground font-medium"
                                } else {
                                    "text-muted-foreground"
                                },
                            ),
                            {t!("layout_root_asidebar_nav_about")}
                        }
                    }
                    Link {
                        to: Route::BlogList {},
                        class: "inline-flex shrink-0",
                        ZyzjIcon { class: format!("w-12 transition-opacity duration-200 {} hover:opacity-100", dim_opacity_class) }
                    }
                }
            }
        }
    }
}
