//! ScrollArea component with custom scrollbars

use dioxus::prelude::*;

stylance::import_style!(style, "scroll_area.module.scss");

/// Scrollbar visibility options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarVisibility {
    /// Always show scrollbars
    Always,
    /// Show on hover/scroll
    #[default]
    Auto,
    /// Never show scrollbars (still scrollable)
    Hidden,
}

/// Scroll direction
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollDirection {
    /// Vertical scrolling only (default)
    #[default]
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both vertical and horizontal scrolling
    Both,
}

/// A scrollable container with custom styled scrollbars
#[component]
pub fn ScrollArea(
    /// Content to scroll
    children: Element,
    /// Maximum height (CSS value like "300px", "50vh")
    #[props(optional, into)]
    max_height: Option<String>,
    /// Maximum width (CSS value)
    #[props(optional, into)]
    max_width: Option<String>,
    /// Fixed height
    #[props(optional, into)]
    height: Option<String>,
    /// Fixed width
    #[props(optional, into)]
    width: Option<String>,
    /// Scroll direction
    #[props(default)]
    direction: ScrollDirection,
    /// Scrollbar visibility
    #[props(default)]
    scrollbar: ScrollbarVisibility,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let direction_class = match direction {
        ScrollDirection::Vertical => style::vertical,
        ScrollDirection::Horizontal => style::horizontal,
        ScrollDirection::Both => style::both,
    };

    let visibility_class = match scrollbar {
        ScrollbarVisibility::Always => style::scrollbar_always,
        ScrollbarVisibility::Auto => style::scrollbar_auto,
        ScrollbarVisibility::Hidden => style::scrollbar_hidden,
    };

    let mut inline_style = String::new();
    if let Some(h) = height {
        inline_style.push_str(&format!("height: {h};"));
    }
    if let Some(w) = width {
        inline_style.push_str(&format!("width: {w};"));
    }
    if let Some(mh) = max_height {
        inline_style.push_str(&format!("max-height: {mh};"));
    }
    if let Some(mw) = max_width {
        inline_style.push_str(&format!("max-width: {mw};"));
    }

    rsx! {
        div {
            class: stylance::classes!(
                style::scroll_area,
                direction_class,
                visibility_class,
                class.as_deref().unwrap_or("")
            ),
            style: inline_style,
            tabindex: "0",

            div { class: style::viewport,
                {children}
            }
        }
    }
}
