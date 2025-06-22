use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto space-y-6 sm:space-y-8",

            // 标题
            div {
                class: "mb-6 sm:mb-8 pb-4 sm:pb-6 border-b border-border/30",
                h1 {
                    class: "text-lg sm:text-xl font-medium text-foreground mb-2 sm:mb-3",
                    { t!("page-about-title") }
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    { t!("page-about-description") }
                }
            }

            // 简单介绍
            div {
                class: "space-y-4 sm:space-y-6",
                p {
                    class: "text-sm sm:text-base text-foreground leading-relaxed",
                    { t!("page-about-intro") }
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    { t!("page-about-blog-description") }
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    { t!("page-about-tech-stack") }
                }
            }

            // 联系方式
            div {
                class: "pt-6 sm:pt-8 border-t border-border/30",
                h2 {
                    class: "text-sm sm:text-base font-medium text-foreground mb-3 sm:mb-4",
                    { t!("page-about-contact") }
                }
                div {
                    class: "space-y-3 sm:space-y-4",
                    div {
                        class: "text-sm sm:text-base text-muted-foreground",
                        "Email: hello@example.com"
                    }
                    div {
                        a {
                            href: "https://github.com",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center gap-2 text-sm sm:text-base text-foreground hover:text-primary transition-colors underline-offset-4 hover:underline min-h-[44px] sm:min-h-auto",
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}
