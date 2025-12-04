//! Badge component for status indicators and labels

use dioxus::prelude::*;

stylance::import_style!(style, "badge.module.scss");

/// Visual style variants for Badge
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    /// Default/neutral badge style
    #[default]
    Default,
    /// Primary/accent colored badge
    Primary,
    /// Success/positive badge (green)
    Success,
    /// Warning badge (yellow/orange)
    Warning,
    /// Error/danger badge (red)
    Error,
}

/// Size variants for Badge
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeSize {
    /// Small badge
    Small,
    /// Medium badge (default)
    #[default]
    Medium,
    /// Large badge
    Large,
}

#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(default)] size: BadgeSize,
    children: Element,
) -> Element {
    let variant_class = match variant {
        BadgeVariant::Default => style::default,
        BadgeVariant::Primary => style::primary,
        BadgeVariant::Success => style::success,
        BadgeVariant::Warning => style::warning,
        BadgeVariant::Error => style::error,
    };

    let size_class = match size {
        BadgeSize::Small => style::small,
        BadgeSize::Medium => "",
        BadgeSize::Large => style::large,
    };

    rsx! {
        span { class: stylance::classes!(style::badge, variant_class, size_class),
            {children}
        }
    }
}
