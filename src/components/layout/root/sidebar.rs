use crate::{
    components::common::{
        locale_switcher::LocaleSwitcher, svgs::LogoSVG, theme_switcher::ThemeSwitcher,
    },
    root::Route,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn RootAsidebar(current_route: Route) -> Element {
    rsx! {
        aside {
            class: "w-48 shrink-0 sticky top-8 space-y-4",
            Link {
                to: Route::BlogList {},
                class: "inline-flex items-center",
                LogoSVG {}
            }

            div {
                input {
                    class: "w-full h-8 px-2 text-sm bg-transparent text-muted-foreground focus:outline-none",
                    r#type: "text",
                    placeholder: "Search..."
                }
            }

            div {
                class: "flex items-center gap-1",
                ThemeSwitcher { is_mobile: false }
                LocaleSwitcher {}
            }

            nav {
                class: "space-y-2 text-sm",
                Link {
                    to: Route::BlogList {},
                    class: format!(
                        "block transition-colors {}",
                        if matches!(current_route, Route::BlogList { .. } | Route::BlogPost { .. }) {
                            "text-foreground font-medium"
                        } else {
                            "text-muted-foreground hover:text-foreground"
                        }
                    ),
                    { t!("layout_root_asidebar_nav_articles") }
                }
                Link {
                    to: Route::TagList {},
                    class: format!(
                        "block transition-colors {}",
                        if matches!(current_route, Route::TagList { .. } | Route::BlogByTag { .. }) {
                            "text-foreground font-medium"
                        } else {
                            "text-muted-foreground hover:text-foreground"
                        }
                    ),
                    { t!("layout_root_asidebar_nav_tags") }
                }
                Link {
                    to: Route::About {},
                    class: format!(
                        "block transition-colors {}",
                        if matches!(current_route, Route::About { .. }) {
                            "text-foreground font-medium"
                        } else {
                            "text-muted-foreground hover:text-foreground"
                        }
                    ),
                    { t!("layout_root_asidebar_nav_about") }
                }
            }
        }
    }
}
