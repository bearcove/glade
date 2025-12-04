//! Page navigation component for prev/next links

use dioxus::prelude::*;

use crate::components::icons::{IconChevronLeft, IconChevronRight};

stylance::import_style!(style, "page_nav.module.scss");

/// Page navigation with previous/next links
#[component]
pub fn PageNav(
    /// Previous page label
    #[props(default)]
    prev_label: String,
    /// Previous page href
    #[props(default)]
    prev_href: String,
    /// Next page label
    #[props(default)]
    next_label: String,
    /// Next page href
    #[props(default)]
    next_href: String,
) -> Element {
    let has_prev = !prev_href.is_empty();
    let has_next = !next_href.is_empty();

    rsx! {
        nav { class: style::page_nav, aria_label: "Page navigation",
            if has_prev {
                a { class: style::link, href: "{prev_href}",
                    span { class: style::icon, IconChevronLeft {} }
                    span { class: style::text,
                        span { class: style::direction, "Previous" }
                        span { class: style::label, "{prev_label}" }
                    }
                }
            } else {
                div { class: style::spacer }
            }
            if has_next {
                a { class: stylance::classes!(style::link, style::next), href: "{next_href}",
                    span { class: style::text,
                        span { class: style::direction, "Next" }
                        span { class: style::label, "{next_label}" }
                    }
                    span { class: style::icon, IconChevronRight {} }
                }
            }
        }
    }
}
