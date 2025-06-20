use crate::models::tag::Tag;
use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-background",

            div {
                class: "container mx-auto px-4 py-8 text-center",

                // 404 图标
                div {
                    class: "mb-8",
                    svg {
                        class: "w-32 h-32 mx-auto text-muted-foreground/40",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1",
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M16 16s-1.5-2-4-2-4 2-4 2" }
                        line { x1: "9", y1: "9", x2: "9.01", y2: "9" }
                        line { x1: "15", y1: "9", x2: "15.01", y2: "9" }
                    }
                }

                // 错误信息
                div {
                    class: "max-w-2xl mx-auto mb-8",
                    h1 {
                        class: "text-6xl font-bold text-primary mb-4",
                        "404"
                    }
                    h2 {
                        class: "text-3xl font-bold text-foreground mb-4",
                        { t!("page-not-found-title") }
                    }
                    p {
                        class: "text-lg text-muted-foreground leading-relaxed mb-6",
                        { t!("page-not-found-message") }
                    }

                    // 显示尝试访问的路径
                    if !route.is_empty() {
                        div {
                            class: "bg-card/60 border border-border rounded-lg p-4 mb-6",
                            p {
                                class: "text-sm text-muted-foreground mb-2",
                                "Attempted path:"
                            }
                            code {
                                class: "text-primary font-mono bg-primary/10 px-2 py-1 rounded",
                                "/{route.join(\"/\")}"
                            }
                        }
                    }
                }

                // 操作按钮
                div {
                    class: "flex flex-col sm:flex-row items-center justify-center gap-4",

                    Link {
                        to: Route::BlogList {},
                        class: "inline-flex items-center px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors duration-200 font-medium",
                        svg {
                            class: "w-5 h-5 mr-2",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                            polyline { points: "9,22 9,12 15,12 15,22" }
                        }
                        { t!("page-not-found-back-home") }
                    }

                    Link {
                        to: Route::BlogList {},
                        class: "inline-flex items-center px-6 py-3 bg-secondary text-secondary-foreground rounded-lg hover:bg-secondary/80 transition-colors duration-200 font-medium",
                        svg {
                            class: "w-5 h-5 mr-2",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14,2 14,8 20,8" }
                            line { x1: "16", y1: "13", x2: "8", y2: "13" }
                            line { x1: "16", y1: "17", x2: "8", y2: "17" }
                            polyline { points: "10,9 9,9 8,9" }
                        }
                        { t!("main-layout-nav-articles") }
                    }
                }

                // 建议链接
                div {
                    class: "mt-12 pt-8 border-t border-border/50",
                    h3 {
                        class: "text-lg font-semibold text-foreground mb-4",
                        "Suggested pages"
                    }
                    div {
                        class: "flex flex-wrap justify-center gap-4",
                        Link {
                            to: Route::About {},
                            class: "text-primary hover:text-primary/80 transition-colors duration-200 text-sm font-medium",
                            { t!("page-about-title") }
                        }
                        span {
                            class: "text-muted-foreground",
                            "•"
                        }
                        Link {
                            to: Route::BlogByTag { tag: Tag::Technology.to_string() },
                            class: "text-primary hover:text-primary/80 transition-colors duration-200 text-sm font-medium",
                            { t!(Tag::Technology.i18n_key()) }
                        }
                        span {
                            class: "text-muted-foreground",
                            "•"
                        }
                        Link {
                            to: Route::BlogByTag { tag: Tag::Life.to_string() },
                            class: "text-primary hover:text-primary/80 transition-colors duration-200 text-sm font-medium",
                            { t!(Tag::Life.i18n_key()) }
                        }
                    }
                }
            }
        }
    }
}
