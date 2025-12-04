//! Pulsing dots loading indicator

use dioxus::prelude::*;

stylance::import_style!(style, "pulsing_dots.module.scss");

/// Size variants for the pulsing dots
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PulsingDotsSize {
    /// Small dots (3px)
    Small,
    /// Medium dots (4px) - default
    #[default]
    Medium,
    /// Large dots (6px)
    Large,
}

/// Pulsing dots loading indicator - three dots that pulse in sequence
#[component]
pub fn PulsingDots(
    /// Size of the dots
    #[props(default)]
    size: PulsingDotsSize,
) -> Element {
    let size_class = match size {
        PulsingDotsSize::Small => style::small,
        PulsingDotsSize::Medium => style::medium,
        PulsingDotsSize::Large => style::large,
    };

    rsx! {
        span { class: stylance::classes!(style::dots, size_class),
            span { class: style::dot }
            span { class: style::dot }
            span { class: style::dot }
        }
    }
}
