//! SidebarLayout - flex layout for sidebar + main content

use dioxus::prelude::*;

stylance::import_style!(style, "sidebar_layout.module.scss");

/// SidebarLayout - arranges sidebar and main content horizontally
#[component]
pub fn SidebarLayout(children: Element) -> Element {
    rsx! {
        div { class: style::layout,
            {children}
        }
    }
}
