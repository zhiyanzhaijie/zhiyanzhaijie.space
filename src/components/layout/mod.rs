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

            // 顶部导航区域 - 简约设计
            header {
                class: "border-b border-border/30 px-10 pb-2 pt-18",

                nav {
                    class: "container flex justify-between px-30",

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
                class: "container mx-auto px-10 py-8",
                Outlet::<Route> {}
            }
        }
    }
}
