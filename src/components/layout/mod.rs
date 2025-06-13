use crate::{components::common::ThemeSwitcher, routes::Route};
use dioxus::prelude::*;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-background text-foreground font-sans",

            // 顶部导航区域 - 简约设计
            header {
                class: "border-b border-border/30 py-8 pt-18",

                nav {
                    class: "container flex justify-between px-30",

                    div {
                        class: "flex items-center justify-start space-x-4",

                        Link {
                            to: Route::Home {},
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors duration-200 relative group",
                            "首页"
                            span {
                                class: "absolute -bottom-1 left-0 w-0 h-px bg-foreground transition-all duration-200 group-hover:w-full"
                            }
                        }

                        Link {
                            to: Route::BlogList {},
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors duration-200 relative group",
                            "文章"
                            span {
                                class: "absolute -bottom-1 left-0 w-0 h-px bg-foreground transition-all duration-200 group-hover:w-full"
                            }
                        }

                        Link {
                            to: Route::TagList {},
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors duration-200 relative group",
                            "标签"
                            span {
                                class: "absolute -bottom-1 left-0 w-0 h-px bg-foreground transition-all duration-200 group-hover:w-full"
                            }
                        }

                        Link {
                            to: Route::About {},
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors duration-200 relative group",
                            "关于"
                            span {
                                class: "absolute -bottom-1 left-0 w-0 h-px bg-foreground transition-all duration-200 group-hover:w-full"
                            }
                        }
                    }

                    div {
                      ThemeSwitcher {}
                    }
                }
            }

            // 主要内容区域
            main {
                class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
        }
    }
}
