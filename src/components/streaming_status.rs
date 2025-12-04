//! Streaming status / activity indicator component

use dioxus::prelude::*;

stylance::import_style!(style, "streaming_status.module.scss");

/// Variant for different streaming activities
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamingStatusVariant {
    /// Generic thinking indicator
    #[default]
    Thinking,
    /// Web search indicator
    Searching,
    /// Tool/command running indicator
    Running,
    /// Writing/generating indicator
    Writing,
    /// Custom (uses provided text)
    Custom,
}

/// Streaming status indicator with animated dots
#[component]
pub fn StreamingStatus(
    /// Status variant
    #[props(default)]
    variant: StreamingStatusVariant,
    /// Custom text (used when variant is Custom)
    #[props(default)]
    text: String,
    /// Whether to show the spinner icon
    #[props(default = true)]
    show_icon: bool,
) -> Element {
    let display_text = match variant {
        StreamingStatusVariant::Thinking => "Thinking".to_string(),
        StreamingStatusVariant::Searching => "Searching".to_string(),
        StreamingStatusVariant::Running => "Running".to_string(),
        StreamingStatusVariant::Writing => "Writing".to_string(),
        StreamingStatusVariant::Custom => text,
    };

    rsx! {
        span { class: style::status,
            if show_icon {
                span { class: style::icon,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83" }
                    }
                }
            }
            span { class: style::text, "{display_text}" }
            span { class: style::dots,
                span { class: style::dot }
                span { class: style::dot }
                span { class: style::dot }
            }
        }
    }
}
