//! Row - horizontal flex layout component

use dioxus::prelude::*;

stylance::import_style!(style, "row.module.scss");

/// Gap size for Row
#[derive(Clone, Copy, PartialEq, Default)]
pub enum RowGap {
    /// No gap between items
    None,
    /// Small gap (8px)
    Small,
    /// Medium gap (16px, default)
    #[default]
    Medium,
    /// Large gap (24px)
    Large,
}

/// Vertical alignment for Row
#[derive(Clone, Copy, PartialEq, Default)]
pub enum RowAlign {
    /// Align items to start (default)
    #[default]
    Start,
    /// Center items vertically
    Center,
    /// Align items to end
    End,
    /// Stretch items to fill container
    Stretch,
}

/// Row - arranges children horizontally with consistent spacing
#[component]
pub fn Row(
    #[props(default)] gap: RowGap,
    #[props(default)] align: RowAlign,
    #[props(default = true)] wrap: bool,
    children: Element,
) -> Element {
    let gap_class = match gap {
        RowGap::None => "",
        RowGap::Small => style::gap_small,
        RowGap::Medium => style::gap_medium,
        RowGap::Large => style::gap_large,
    };

    let align_class = match align {
        RowAlign::Start => "",
        RowAlign::Center => style::align_center,
        RowAlign::End => style::align_end,
        RowAlign::Stretch => style::align_stretch,
    };

    let wrap_class = if wrap { style::wrap } else { "" };

    rsx! {
        div { class: stylance::classes!(style::row, gap_class, align_class, wrap_class),
            {children}
        }
    }
}
