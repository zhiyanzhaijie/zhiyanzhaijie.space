use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AboutView() -> Element {
    rsx! {
        div {
            class: "max-w-2xl mx-auto space-y-8",

            div {
                h1 {
                    class: "text-xl sm:text-2xl font-semibold tracking-tight text-foreground mb-2",
                    { t!("page-about-title") }
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    { t!("page-about-description") }
                }
            }

            div {
                class: "space-y-4",
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

            div {
                h2 {
                    class: "text-base font-medium text-foreground mb-3",
                    { t!("page-about-contact") }
                }
                div {
                    class: "space-y-3",
                    div {
                        class: "text-sm sm:text-base text-muted-foreground",
                        "Email: ..."
                    }
                    div {
                        a {
                            href: "https://github.com/zhiyanzhaijie",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center gap-2 text-sm sm:text-base text-muted-foreground hover:text-foreground transition-colors underline-offset-4 hover:underline min-h-[44px] sm:min-h-auto",
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}
