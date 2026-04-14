use crate::root::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Crumb {
    label: String,
    to: Option<Route>,
}

fn home_crumb() -> Crumb {
    Crumb {
        label: "~".to_string(),
        to: Some(Route::BlogList {}),
    }
}

fn blog_crumb() -> Crumb {
    Crumb {
        label: "blog".to_string(),
        to: Some(Route::BlogList {}),
    }
}

fn tags_crumb() -> Crumb {
    Crumb {
        label: "tags".to_string(),
        to: Some(Route::TagList {}),
    }
}

fn build_crumbs(route: &Route) -> Vec<Crumb> {
    match route {
        Route::BlogList {} => vec![home_crumb()],
        Route::BlogPost { slug } => vec![
            home_crumb(),
            blog_crumb(),
            Crumb {
                label: slug.clone(),
                to: None,
            },
        ],
        Route::TagList {} => vec![
            home_crumb(),
            Crumb {
                label: "tags".to_string(),
                to: None,
            },
        ],
        Route::TagsTag { tag } => vec![
            home_crumb(),
            tags_crumb(),
            Crumb {
                label: tag.clone(),
                to: None,
            },
        ],
        Route::About {} => vec![
            home_crumb(),
            Crumb {
                label: "about".to_string(),
                to: None,
            },
        ],
        Route::PageNotFound { .. } => vec![
            home_crumb(),
            Crumb {
                label: "404".to_string(),
                to: None,
            },
        ],
    }
}

#[component]
pub fn CommonBreadcrumb(
    #[props(default = false)] compact: bool,
    #[props(default = "/".to_string())] separator: String,
    #[props(default = String::new())] class: String,
) -> Element {
    let dim_opacity_class = "opacity-50";
    let route = use_route::<Route>();
    let crumbs = build_crumbs(&route);
    let total = crumbs.len();

    rsx! {
        nav {
            aria_label: "breadcrumb",
            class: "flex items-center",
            class: if compact { "text-xs" } else { "text-sm" },
            class: "{class}",
            ol { class: if compact {
                    "flex items-center gap-1 text-xs leading-none text-muted-foreground whitespace-nowrap"
                } else {
                    "flex flex-wrap items-center gap-1.5 text-muted-foreground"
                },
                for (idx, crumb) in crumbs.into_iter().enumerate() {
                    li { key: "{crumb.label}-{idx}", class: "inline-flex items-center gap-1.5",
                        if idx > 0 {
                            span { class: "select-none {dim_opacity_class}", "{separator}" }
                        }
                        if idx + 1 == total {
                            span {
                                aria_current: "page",
                                class: "text-foreground {dim_opacity_class}",
                                "{crumb.label}"
                            }
                        } else if let Some(to) = crumb.to {
                            Link {
                                to: to,
                                class: "transition-colors transition-opacity duration-200 {dim_opacity_class} hover:text-foreground hover:opacity-100 hover:underline underline-offset-4",
                                "{crumb.label}"
                            }
                        } else {
                            span { class: "{dim_opacity_class}", "{crumb.label}" }
                        }
                    }
                }
            }
        }
    }
}
