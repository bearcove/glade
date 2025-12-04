//! Descriptions component for key-value pairs display

use dioxus::prelude::*;

stylance::import_style!(style, "descriptions.module.scss");

/// Layout direction for descriptions
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DescriptionsLayout {
    /// Label and value side by side (default)
    #[default]
    Horizontal,
    /// Label above value
    Vertical,
}

/// Size variants
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DescriptionsSize {
    /// Small description items
    Small,
    /// Medium description items (default)
    #[default]
    Medium,
    /// Large description items
    Large,
}

/// A description list for displaying key-value pairs
#[component]
pub fn Descriptions(
    /// Description items
    children: Element,
    /// Optional title
    #[props(optional, into)]
    title: Option<String>,
    /// Number of columns (1-4)
    #[props(default = 3)]
    columns: usize,
    /// Layout direction
    #[props(default)]
    layout: DescriptionsLayout,
    /// Size variant
    #[props(default)]
    size: DescriptionsSize,
    /// Whether to show borders
    #[props(default = true)]
    bordered: bool,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let cols = columns.clamp(1, 4);

    let size_class = match size {
        DescriptionsSize::Small => style::small,
        DescriptionsSize::Medium => "",
        DescriptionsSize::Large => style::large,
    };

    let layout_class = match layout {
        DescriptionsLayout::Horizontal => style::horizontal,
        DescriptionsLayout::Vertical => style::vertical,
    };

    rsx! {
        div {
            class: stylance::classes!(
                style::descriptions,
                size_class,
                layout_class,
                if bordered { style::bordered } else { "" },
                class.as_deref().unwrap_or("")
            ),
            style: format!("--desc-columns: {cols};"),

            if let Some(title_text) = title {
                div { class: style::title, "{title_text}" }
            }

            dl { class: style::list,
                {children}
            }
        }
    }
}

/// A single description item (key-value pair)
#[component]
pub fn DescriptionItem(
    /// The label/key
    #[props(into)]
    label: String,
    /// The value content
    children: Element,
    /// Span multiple columns (1-4)
    #[props(default = 1)]
    span: usize,
) -> Element {
    let col_span = span.clamp(1, 4);

    rsx! {
        div {
            class: style::item,
            style: if col_span > 1 { format!("grid-column: span {col_span};") } else { String::new() },

            dt { class: style::label, "{label}" }
            dd { class: style::value, {children} }
        }
    }
}
