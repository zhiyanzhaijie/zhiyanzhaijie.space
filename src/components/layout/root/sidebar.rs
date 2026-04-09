use crate::{
    components::common::{
        layout_cell::{LayoutCell, LayoutCellPadding},
        locale_switcher::LocaleSwitcher,
        svgs::{LogoSVG, MenuSVG},
        theme_switcher::ThemeSwitcher,
    },
    root::Route,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn RootAsidebar(current_route: Route) -> Element {
    let mut is_mobile_nav_open = use_signal(|| false);
    let is_articles_route = matches!(
        current_route,
        Route::BlogList { .. } | Route::BlogPost { .. }
    );
    let is_tags_route = matches!(
        current_route,
        Route::TagList { .. } | Route::BlogByTag { .. }
    );
    let is_about_route = matches!(current_route, Route::About { .. });
    rsx! {
        aside {
            class: "w-full md:w-56 shrink-0 md:fixed md:left-[max(1.5rem,calc((100vw-80rem)/2+1.5rem))] md:top-0 md:z-20",
            LayoutCell {
                padding: LayoutCellPadding::Normal,
                div {
                    class: "space-y-4",
                    div {
                        class: "flex items-center justify-between md:block",
                        Link {
                            to: Route::BlogList {},
                            class: "inline-flex items-center",
                            LogoSVG {}
                        }

                        div {
                            class: "flex items-center gap-1 md:hidden",
                            ThemeSwitcher { is_mobile: true }
                            LocaleSwitcher {}

                            div {
                                class: "relative",
                                button {
                                    class: "w-8 h-8 flex items-center justify-center rounded focus:outline-none cursor-pointer text-muted-foreground hover:text-foreground transition-colors",
                                    title: "Open menu",
                                    onclick: move |_| is_mobile_nav_open.set(!is_mobile_nav_open()),
                                    MenuSVG {}
                                }

                                if is_mobile_nav_open() {
                                    nav {
                                        class: "absolute right-0 mt-3 w-52 space-y-2 rounded-md border border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-900 p-2 text-sm shadow-md backdrop-blur-md z-20",
                                        Link {
                                            to: Route::BlogList {},
                                            class: "block",
                                            onclick: move |_| is_mobile_nav_open.set(false),
                                            span {
                                                class: "transition-colors",
                                                class: if is_articles_route {
                                                    "text-foreground font-medium"
                                                } else {
                                                    "text-muted-foreground hover:text-foreground"
                                                },
                                                { t!("layout_root_asidebar_nav_articles") }
                                            }
                                        }
                                        Link {
                                            to: Route::TagList {},
                                            class: "block",
                                            onclick: move |_| is_mobile_nav_open.set(false),
                                            span {
                                                class: "transition-colors",
                                                class: if is_tags_route {
                                                    "text-foreground font-medium"
                                                } else {
                                                    "text-muted-foreground hover:text-foreground"
                                                },
                                                { t!("layout_root_asidebar_nav_tags") }
                                            }
                                        }
                                        Link {
                                            to: Route::About {},
                                            class: "block",
                                            onclick: move |_| is_mobile_nav_open.set(false),
                                            span {
                                                class: "transition-colors",
                                                class: if is_about_route {
                                                    "text-foreground font-medium"
                                                } else {
                                                    "text-muted-foreground hover:text-foreground"
                                                },
                                                { t!("layout_root_asidebar_nav_about") }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "hidden md:block",
                        input {
                            class: "w-full h-8 px-2 text-sm bg-transparent text-muted-foreground focus:outline-none",
                            r#type: "text",
                            placeholder: "Search..."
                        }
                    }

                    div {
                        class: "hidden md:flex items-center gap-1",
                        ThemeSwitcher { is_mobile: false }
                        LocaleSwitcher {}
                    }

                    nav {
                        class: "hidden md:block space-y-2 text-sm",
                        Link {
                            to: Route::BlogList {},
                            class: "block",
                            span {
                                class: "transition-colors",
                                class: if is_articles_route {
                                    "text-foreground font-medium"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                },
                                { t!("layout_root_asidebar_nav_articles") }
                            }
                        }
                        Link {
                            to: Route::TagList {},
                            class: "block",
                            span {
                                class: "transition-colors",
                                class: if is_tags_route {
                                    "text-foreground font-medium"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                },
                                { t!("layout_root_asidebar_nav_tags") }
                            }
                        }
                        Link {
                            to: Route::About {},
                            class: "block",
                            span {
                                class: "transition-colors",
                                class: if is_about_route {
                                    "text-foreground font-medium"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                },
                                { t!("layout_root_asidebar_nav_about") }
                            }
                        }
                    }
                }
            }
        }
    }
}
