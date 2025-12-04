//! Site footer component with links and sections

use dioxus::prelude::*;

stylance::import_style!(#[allow(dead_code)] style, "footer.module.scss");

/// Footer - site footer component
#[component]
pub fn Footer(
    /// Footer content
    children: Element,
) -> Element {
    rsx! {
        footer { class: style::footer,
            div { class: style::container, {children} }
        }
    }
}

/// FooterSection - a column section in the footer
#[component]
pub fn FooterSection(
    /// Section title
    title: Option<String>,
    /// Section content
    children: Element,
) -> Element {
    rsx! {
        div { class: style::section,
            if let Some(t) = title {
                h4 { class: style::section_title, "{t}" }
            }
            div { class: style::section_content, {children} }
        }
    }
}

/// FooterLinks - a list of links in a footer section
#[component]
pub fn FooterLinks(
    /// Link items
    children: Element,
) -> Element {
    rsx! {
        ul { class: style::links, {children} }
    }
}

/// FooterLink - a single link in the footer
#[component]
pub fn FooterLink(
    /// Link URL
    href: String,
    /// Whether this is an external link
    #[props(default = false)]
    external: bool,
    /// Link text
    children: Element,
) -> Element {
    let target = if external { Some("_blank") } else { None };
    let rel = if external { Some("noopener noreferrer") } else { None };

    rsx! {
        li { class: style::link_item,
            a { href: "{href}", target, rel, class: style::link, {children} }
        }
    }
}

/// FooterBottom - the bottom row of the footer (typically copyright)
#[component]
pub fn FooterBottom(
    /// Bottom content
    children: Element,
) -> Element {
    rsx! {
        div { class: style::bottom, {children} }
    }
}

/// FooterCopyright - copyright notice
#[component]
pub fn FooterCopyright(
    /// Year or year range (e.g., "2024" or "2020-2024")
    year: String,
    /// Company/owner name
    holder: String,
) -> Element {
    rsx! {
        p { class: style::copyright, "© {year} {holder}. All rights reserved." }
    }
}
