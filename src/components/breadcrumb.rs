//! Breadcrumb navigation component

use dioxus::prelude::*;

stylance::import_style!(style, "breadcrumb.module.scss");

/// A single breadcrumb item
#[component]
pub fn BreadcrumbItem(
    /// The href for the breadcrumb link (if not provided, renders as text)
    #[props(default)]
    href: String,
    /// Whether this is the current/active page
    #[props(default = false)]
    current: bool,
    /// Children content
    children: Element,
) -> Element {
    let class = if current {
        stylance::classes!(style::item, style::current)
    } else {
        style::item.to_string()
    };

    let has_href = !href.is_empty();

    rsx! {
        li { class: "{class}",
            if has_href && !current {
                a { class: style::link, href: "{href}", {children} }
            } else {
                span {
                    class: style::link,
                    aria_current: if current { Some("page") } else { None },
                    {children}
                }
            }
        }
    }
}

/// Breadcrumb navigation container
#[component]
pub fn Breadcrumb(
    /// Custom separator (defaults to "/")
    #[props(default = "/".to_string())]
    separator: String,
    /// Children (should be BreadcrumbItem components)
    children: Element,
) -> Element {
    let separator_style = format!("--separator: '{}';", separator);

    rsx! {
        nav { class: style::breadcrumb, aria_label: "Breadcrumb",
            ol { class: style::list, style: "{separator_style}",
                {children}
            }
        }
    }
}
