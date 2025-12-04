//! Section - content section with title and optional subsections

use dioxus::prelude::*;

stylance::import_style!(style, "section.module.scss");

/// Section - a content section with a title
#[component]
pub fn Section(
    /// Section ID for anchor links
    #[props(default)]
    id: String,
    /// Section title
    title: String,
    children: Element,
) -> Element {
    rsx! {
        section {
            id: if !id.is_empty() { Some(id) } else { None },
            class: style::section,
            h2 { class: style::title, "{title}" }
            {children}
        }
    }
}

/// SubSection - a subsection within a Section
#[component]
pub fn SubSection(
    /// Subsection title
    title: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: style::subsection,
            h3 { class: style::subtitle, "{title}" }
            {children}
        }
    }
}
