use crate::{
    components::common::{
        locale_switcher::LocaleSwitcher,
        svgs::{LogoSVG, MenuSVG},
        theme_switcher::ThemeSwitcher,
    },
    routes::Route,
};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_router::prelude::use_route;

#[component]
pub fn MainLayout() -> Element {
    let current_route = use_route::<Route>();
    let mut is_menu_open = use_signal(|| false);

    rsx! {
        div {
            class: "min-h-screen bg-background text-foreground font-sans",

            header {
                class: "sticky top-0 z-50 w-full bg-background/90 px-4 sm:px-6 md:px-8 lg:px-12 pb-2 pt-4 sm:pt-6 md:pt-8 lg:pt-12 relative",

                div {
                    class: "absolute -bottom-15 left-0 w-full h-15 bg-gradient-to-b from-background/90 to-transparent pointer-events-none z-50"
                }

                nav {
                    class: "w-full flex justify-between items-center",

                    div {
                        class: "flex items-center justify-start space-x-2 sm:space-x-4 flex-wrap gap-y-2",

                        Link {
                            class: "mr-4",
                            to: Route::BlogList {},
                            LogoSVG  {  }
                        }

                        Link {
                            to: Route::BlogList {},
                            class: format!(
                                "text-xs sm:text-sm font-medium transition-colors duration-200 relative group whitespace-nowrap {}",
                                if matches!(current_route, Route::BlogList { .. } | Route::BlogPost { .. } ) {
                                    "text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            { t!("main-layout-nav-articles") }
                            span {
                                class: format!(
                                    "absolute -bottom-1 left-0 h-px bg-foreground transition-all duration-200 {}",
                                    if matches!(current_route, Route::BlogList { .. } | Route::BlogPost { .. }) {
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
                                "text-xs sm:text-sm font-medium transition-colors duration-200 relative group whitespace-nowrap {}",
                                if matches!(current_route, Route::TagList { .. } | Route::BlogByTag { .. }) {
                                    "text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            { t!("main-layout-nav-tags") }
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
                                "text-xs sm:text-sm font-medium transition-colors duration-200 relative group whitespace-nowrap {}",
                                if matches!(current_route, Route::About { .. }) {
                                    "text-foreground"
                                } else {
                                    "text-muted-foreground hover:text-foreground"
                                }
                            ),
                            { t!("main-layout-nav-about") }
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

                    // 桌面端：显示切换按钮
                    div {
                        class: "hidden sm:flex items-center gap-2",
                        ThemeSwitcher { is_mobile: false }
                        LocaleSwitcher {}
                    }

                    // 移动端：显示菜单按钮
                    button {
                        class: "sm:hidden flex items-center justify-center w-10 h-10 rounded-md hover:bg-muted/50 transition-colors",
                        onclick: move |_| {
                            let current = *is_menu_open.read();
                            is_menu_open.set(!current);
                        },
                        MenuSVG {}
                    }
                }
            }

            // 移动端折叠菜单
            if *is_menu_open.read() {
                div {
                    class: "sm:hidden fixed inset-0 z-99",
                    // 背景遮罩
                    div {
                        class: "absolute inset-0",
                        onclick: move |_| {
                            is_menu_open.set(false);
                        },
                    }
                    // 菜单面板
                    div {
                        class: "absolute top-16 right-4 bg-background border border-border rounded-lg shadow-lg p-1",
                        div {
                            class: "flex flex-col gap-1",
                            ThemeSwitcher { is_mobile: true }
                            LocaleSwitcher {}
                        }
                    }
                }
            }

            // 主要内容区域
            main {
                class: "container mx-auto px-4 sm:px-6 md:px-8 lg:px-10 py-4 sm:py-6 md:py-8 relative z-10 max-w-full overflow-x-hidden",
                Outlet::<Route> {}
            }
        }
    }
}
