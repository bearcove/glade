//! Stack - vertical flex layout component

use dioxus::prelude::*;

stylance::import_style!(style, "stack.module.scss");

/// Gap size for Stack
#[derive(Clone, Copy, PartialEq, Default)]
pub enum StackGap {
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

/// Stack - arranges children vertically with consistent spacing
#[component]
pub fn Stack(
    #[props(default)] gap: StackGap,
    children: Element,
) -> Element {
    let gap_class = match gap {
        StackGap::None => "",
        StackGap::Small => style::gap_small,
        StackGap::Medium => style::gap_medium,
        StackGap::Large => style::gap_large,
    };

    rsx! {
        div { class: stylance::classes!(style::stack, gap_class),
            {children}
        }
    }
}
