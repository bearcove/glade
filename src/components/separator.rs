//! Separator/divider component for visual content separation

use dioxus::prelude::*;

stylance::import_style!(style, "separator.module.scss");

/// Orientation of the separator
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    /// Horizontal separator line (default)
    #[default]
    Horizontal,
    /// Vertical separator line
    Vertical,
}

/// A visual separator/divider between content
#[component]
pub fn Separator(
    /// Orientation (horizontal or vertical)
    #[props(default)]
    orientation: SeparatorOrientation,
    /// Optional label text to display in the center
    #[props(optional, into)]
    label: Option<String>,
    /// Whether to use a dashed line style
    #[props(default = false)]
    dashed: bool,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let orientation_class = match orientation {
        SeparatorOrientation::Horizontal => style::horizontal,
        SeparatorOrientation::Vertical => style::vertical,
    };

    rsx! {
        div {
            class: stylance::classes!(
                style::separator,
                orientation_class,
                if dashed { style::dashed } else { "" },
                class.as_deref().unwrap_or("")
            ),
            role: "separator",
            aria_orientation: match orientation {
                SeparatorOrientation::Horizontal => "horizontal",
                SeparatorOrientation::Vertical => "vertical",
            },

            if let Some(text) = label {
                span { class: style::label, "{text}" }
            }
        }
    }
}
