use crate::root::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn NotFoundView(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4",

            div {
                class: "mx-auto py-8 text-center max-w-2xl",

                div {
                    class: "max-w-2xl mx-auto mb-6 sm:mb-8",
                    h1 {
                        class: "text-4xl sm:text-5xl lg:text-6xl font-semibold tracking-tight text-foreground mb-3 sm:mb-4",
                        "404"
                    }
                    h2 {
                        class: "text-xl sm:text-2xl lg:text-3xl font-semibold tracking-tight text-foreground mb-3 sm:mb-4 leading-tight",
                        { t!("page-not-found-title") }
                    }
                    p {
                        class: "text-sm sm:text-base lg:text-lg text-muted-foreground leading-relaxed mb-6",
                        { t!("page-not-found-message") }
                    }

                    if !route.is_empty() {
                        div {
                            class: "mb-6 text-left",
                            p {
                                class: "text-xs sm:text-sm text-muted-foreground mb-2",
                                "Attempted path:"
                            }
                            code {
                                class: "text-foreground font-mono text-xs sm:text-sm break-all",
                                "/{route.join(\"/\")}"
                            }
                        }
                    }
                }

                div {
                    class: "flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4",

                    Link {
                        to: Route::BlogList {},
                        class: "inline-flex items-center px-2 py-2 text-muted-foreground hover:text-foreground transition-colors duration-200 font-medium min-h-[48px] w-full sm:w-auto justify-center underline-offset-4 hover:underline",
                        { t!("page-not-found-back-home") }
                    }

                    Link {
                        to: Route::BlogList {},
                        class: "inline-flex items-center px-2 py-2 text-muted-foreground hover:text-foreground transition-colors duration-200 font-medium min-h-[48px] w-full sm:w-auto justify-center underline-offset-4 hover:underline",
                        { t!("main-layout-nav-articles") }
                    }
                }
            }
        }
    }
}
