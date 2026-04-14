use crate::components::icons::{GithubIcon, LinkedinIcon, TwitterIcon};
use dioxus::prelude::*;
use dioxus_markdown::CustomComponents;

#[component]
pub fn SocialLinkComponent() -> Element {
    rsx! {
        div { class: "flex items-center gap-3",
            Link {
                to: "https://github.com/zhiyanzhaijie",
                rel: "noopener noreferrer",
                class: "text-muted-foreground hover:text-foreground transition-colors",
                GithubIcon { class: "w-5 h-5" }
            }
            Link {
                to: "https://www.linkedin.com/in/corwin-chan-1a575b400",
                rel: "noopener noreferrer",
                class: "text-muted-foreground hover:text-foreground transition-colors",
                LinkedinIcon { class: "w-5 h-5" }
            }
            Link {
                to: "https://x.com/zyzj8",
                rel: "noopener noreferrer",
                class: "text-muted-foreground hover:text-foreground transition-colors",
                TwitterIcon { class: "w-5 h-5" }
            }
        }
    }
}

pub fn registe_md_comp(components: &mut CustomComponents) {
    components.register("SocialLink", |_props| {
        Ok(rsx! {
            SocialLinkComponent {}
        })
    });
    components.register("social_link", |_props| {
        Ok(rsx! {
            SocialLinkComponent {}
        })
    });
}