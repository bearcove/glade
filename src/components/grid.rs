//! Grid - CSS grid layout component

use dioxus::prelude::*;

stylance::import_style!(style, "grid.module.scss");

/// Column configuration for Grid
#[derive(Clone, Copy, PartialEq, Default)]
pub enum GridColumns {
    /// Auto-fit columns with minimum width
    #[default]
    Auto,
    /// Fixed 2 columns
    Two,
    /// Fixed 3 columns
    Three,
    /// Fixed 4 columns
    Four,
}

/// Gap size for Grid
#[derive(Clone, Copy, PartialEq, Default)]
pub enum GridGap {
    /// Small gap (8px)
    Small,
    /// Medium gap (16px, default)
    #[default]
    Medium,
    /// Large gap (24px)
    Large,
}

/// Grid - arranges children in a CSS grid layout
#[component]
pub fn Grid(
    #[props(default)] columns: GridColumns,
    #[props(default)] gap: GridGap,
    children: Element,
) -> Element {
    let columns_class = match columns {
        GridColumns::Auto => style::cols_auto,
        GridColumns::Two => style::cols_2,
        GridColumns::Three => style::cols_3,
        GridColumns::Four => style::cols_4,
    };

    let gap_class = match gap {
        GridGap::Small => style::gap_small,
        GridGap::Medium => style::gap_medium,
        GridGap::Large => style::gap_large,
    };

    rsx! {
        div { class: stylance::classes!(style::grid, columns_class, gap_class),
            {children}
        }
    }
}
