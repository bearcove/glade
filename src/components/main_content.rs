//! MainContent - main content area that works with Sidebar

use dioxus::prelude::*;

stylance::import_style!(style, "main_content.module.scss");

/// MainContent - the main content area in a sidebar layout
#[component]
pub fn MainContent(children: Element) -> Element {
    rsx! {
        main { class: style::main,
            {children}
        }
    }
}
