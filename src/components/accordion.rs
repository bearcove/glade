//! Accordion component for collapsible content sections

use dioxus::prelude::*;

use crate::IconChevronDown;

stylance::import_style!(style, "accordion.module.scss");

/// A single accordion item with header and collapsible content
#[component]
pub fn AccordionItem(
    /// The header/trigger content
    title: String,
    /// Whether this item is initially open
    #[props(default = false)]
    default_open: bool,
    /// Whether this item is disabled
    #[props(default = false)]
    disabled: bool,
    /// Children content (the collapsible body)
    children: Element,
) -> Element {
    let mut is_open = use_signal(|| default_open);

    let item_class = if is_open() {
        if disabled {
            stylance::classes!(style::item, style::open, style::disabled)
        } else {
            stylance::classes!(style::item, style::open)
        }
    } else if disabled {
        stylance::classes!(style::item, style::disabled)
    } else {
        style::item.to_string()
    };

    rsx! {
        div { class: "{item_class}",
            button {
                class: style::trigger,
                r#type: "button",
                disabled,
                aria_expanded: "{is_open}",
                onclick: move |_| {
                    if !disabled {
                        is_open.set(!is_open());
                    }
                },
                span { class: style::title, "{title}" }
                span { class: style::icon, IconChevronDown {} }
            }
            div {
                class: style::content,
                style: if is_open() { "display: block" } else { "display: none" },
                div { class: style::body, {children} }
            }
        }
    }
}

/// Accordion container for multiple collapsible items
#[component]
pub fn Accordion(
    /// Whether to show borders between items
    #[props(default = false)]
    bordered: bool,
    /// Children (should be AccordionItem components)
    children: Element,
) -> Element {
    let class = if bordered {
        stylance::classes!(style::accordion, style::bordered)
    } else {
        style::accordion.to_string()
    };

    rsx! {
        div { class: "{class}", {children} }
    }
}
