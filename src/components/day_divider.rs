//! Day divider and message separator components

use dioxus::prelude::*;

stylance::import_style!(style, "day_divider.module.scss");

/// Day divider for separating messages by date
#[component]
pub fn DayDivider(
    /// Label to display (e.g., "Today", "Yesterday", "Nov 22, 2024")
    label: String,
) -> Element {
    rsx! {
        div { class: style::divider,
            div { class: style::line }
            span { class: style::label, "{label}" }
            div { class: style::line }
        }
    }
}

/// New messages divider
#[component]
pub fn NewMessagesDivider(
    /// Custom label (defaults to "New")
    #[props(default = "New".to_string())]
    label: String,
) -> Element {
    rsx! {
        div { class: stylance::classes!(style::divider, style::new_messages),
            div { class: style::line }
            span { class: style::label, "{label}" }
            div { class: style::line }
        }
    }
}
