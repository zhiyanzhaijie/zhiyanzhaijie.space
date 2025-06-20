use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "space-y-6",

            // 标题
            div {
                class: "mb-8 pb-6 border-b border-border/30",
                h1 {
                    class: "text-lg font-medium text-foreground mb-2",
                    { t!("page-about-title") }
                }
                p {
                    class: "text-sm text-muted-foreground",
                    { t!("page-about-description") }
                }
            }

            // 简单介绍
            div {
                class: "space-y-4 text-sm leading-relaxed",
                p {
                    class: "text-foreground",
                    { t!("page-about-intro") }
                }
                p {
                    class: "text-muted-foreground",
                    { t!("page-about-blog-description") }
                }
                p {
                    class: "text-muted-foreground",
                    { t!("page-about-tech-stack") }
                }
            }

            // 联系方式
            div {
                class: "pt-6 border-t border-border/30",
                h2 {
                    class: "text-sm font-medium text-foreground mb-3",
                    { t!("page-about-contact") }
                }
                div {
                    class: "space-y-2 text-sm text-muted-foreground",
                    div {
                        "Email: hello@example.com"
                    }
                    div {
                        a {
                            href: "https://github.com",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-foreground hover:text-primary transition-colors",
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}
