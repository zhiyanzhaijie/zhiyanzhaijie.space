use dioxus::prelude::*;

#[component]
pub fn AboutView() -> Element {
    rsx! {
        div {
            class: "max-w-2xl mx-auto space-y-8",

            div {
                h1 {
                    class: "text-xl sm:text-2xl font-semibold tracking-tight text-foreground mb-2",
                    "zhiyanzhaijie"
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    "Programming blog"
                }
            }

            div {
                class: "space-y-4",
                p {
                    class: "text-sm sm:text-base text-foreground leading-relaxed",
                    "Architecture design bachelor, 3 years web experience"
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    "Recently focus on Rust web development."
                }
                p {
                    class: "text-sm sm:text-base text-muted-foreground leading-relaxed",
                    "Experience: webUI, C#, Rust"
                }
            }

            div {
                h2 {
                    class: "text-base font-medium text-foreground mb-3",
                    "Contact"
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
