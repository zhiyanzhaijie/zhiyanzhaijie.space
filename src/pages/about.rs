use dioxus::prelude::*;

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
                    "关于"
                }
                p {
                    class: "text-sm text-muted-foreground",
                    "一些个人信息"
                }
            }

            // 简单介绍
            div {
                class: "space-y-4 text-sm leading-relaxed",
                p {
                    class: "text-foreground",
                    "我是一个喜欢学习和分享的开发者，专注于前端技术和全栈开发。"
                }
                p {
                    class: "text-muted-foreground",
                    "这个博客是我记录学习历程和分享技术心得的地方。主要内容包括技术学习笔记、项目经验分享，以及一些生活感悟。"
                }
                p {
                    class: "text-muted-foreground",
                    "目前主要使用的技术栈：Rust、JavaScript、React、Node.js。"
                }
            }

            // 联系方式
            div {
                class: "pt-6 border-t border-border/30",
                h2 {
                    class: "text-sm font-medium text-foreground mb-3",
                    "联系方式"
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
