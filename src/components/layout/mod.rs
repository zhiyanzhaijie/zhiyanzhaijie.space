use crate::{
    components::common::{LogoSVG, ThemeSwitcher},
    routes::Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_route;

#[component]
pub fn MainLayout() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        div {
            class: "min-h-screen bg-background text-foreground font-sans",

            header {
                class: "sticky top-0 z-50 w-full bg-background/90 px-12 pb-2 pt-12 relative",

                div {
                    class: "absolute -bottom-15 left-0 w-full h-15 bg-gradient-to-b from-background/90 to-transparent pointer-events-none z-50"
                }

                nav {
                    class: "w-full flex justify-between",

                    div {
                        class: "flex items-center justify-start space-x-4",

                        Link {
                            to: Route::BlogList {},

                            LogoSVG {  }
                        }

                        Link {
                            to: Route::BlogList {},
                            class: format!(
                                "text-sm font-medium transition-colors duration-200 relative group {}",
                                if matches!(current_route, Route::BlogList { .. } | Route::BlogPost { .. } | Route::BlogByTag { .. }) {
                                    "text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            "文章"
                            span {
                                class: format!(
                                    "absolute -bottom-1 left-0 h-px bg-foreground transition-all duration-200 {}",
                                    if matches!(current_route, Route::BlogList { .. } | Route::BlogPost { .. } | Route::BlogByTag { .. }) {
                                        "w-full"
                                    } else {
                                        "w-0 group-hover:w-full"
                                    }
                                )
                            }
                        }

                        Link {
                            to: Route::TagList {},
                            class: format!(
                                "text-sm font-medium transition-colors duration-200 relative group {}",
                                if matches!(current_route, Route::TagList { .. }) {
                                    "text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            "标签"
                            span {
                                class: format!(
                                    "absolute -bottom-1 left-0 h-px bg-foreground transition-all duration-200 {}",
                                    if matches!(current_route, Route::TagList { .. }) {
                                        "w-full"
                                    } else {
                                        "w-0 group-hover:w-full"
                                    }
                                )
                            }
                        }

                        Link {
                            to: Route::About {},
                            class: format!(
                                "text-sm font-medium transition-colors duration-200 relative group {}",
                                if matches!(current_route, Route::About { .. }) {
                                    "text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            "关于"
                            span {
                                class: format!(
                                    "absolute -bottom-1 left-0 h-px bg-foreground transition-all duration-200 {}",
                                    if matches!(current_route, Route::About { .. }) {
                                        "w-full"
                                    } else {
                                        "w-0 group-hover:w-full"
                                    }
                                )
                            }
                        }
                    }

                    div {
                      class: "flex justify-end",
                        ThemeSwitcher {}
                    }
                }
            }

            // 主要内容区域
            main {
                class: "container mx-auto px-10 py-8 relative z-10",
                Outlet::<Route> {}
            }
        }
    }
}
