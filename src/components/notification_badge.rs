//! Notification badge component for counts and indicators

use dioxus::prelude::*;

stylance::import_style!(style, "notification_badge.module.scss");

/// Size variants for the NotificationBadge
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationBadgeSize {
    /// Small badge
    Small,
    /// Medium badge (default)
    #[default]
    Medium,
    /// Large badge
    Large,
}

/// Notification badge for counts or dot indicators
#[component]
pub fn NotificationBadge(
    /// Count to display (None = dot only)
    count: Option<u32>,
    /// Maximum count to show (displays "99+" for values above)
    #[props(default = 99)]
    max: u32,
    /// Size variant
    #[props(default)]
    size: NotificationBadgeSize,
    /// Whether to show the badge (useful for hiding when count is 0)
    #[props(default = true)]
    show: bool,
    /// Position relative to parent (for absolute positioning)
    #[props(default = false)]
    positioned: bool,
) -> Element {
    if !show {
        return rsx! {};
    }

    let size_class = match size {
        NotificationBadgeSize::Small => style::small,
        NotificationBadgeSize::Medium => "",
        NotificationBadgeSize::Large => style::large,
    };

    let positioned_class = if positioned { style::positioned } else { "" };

    let content = match count {
        Some(n) if n > max => format!("{max}+"),
        Some(n) => n.to_string(),
        None => String::new(),
    };

    let is_dot = count.is_none();
    let dot_class = if is_dot { style::dot } else { "" };

    rsx! {
        span { class: stylance::classes!(style::badge, size_class, positioned_class, dot_class),
            if !is_dot {
                "{content}"
            }
        }
    }
}

/// Wrapper to position a badge relative to its child
#[component]
pub fn WithBadge(
    /// Badge count (None = dot only)
    count: Option<u32>,
    /// Whether to show the badge
    #[props(default = true)]
    show: bool,
    /// Badge size
    #[props(default)]
    size: NotificationBadgeSize,
    /// Children to wrap
    children: Element,
) -> Element {
    let size_class = match size {
        NotificationBadgeSize::Small => style::small,
        NotificationBadgeSize::Medium => "",
        NotificationBadgeSize::Large => style::large,
    };
    let is_dot = count.is_none();
    let dot_class = if is_dot { style::dot } else { "" };
    let content = match count {
        Some(n) if n > 99 => "99+".to_string(),
        Some(n) => n.to_string(),
        None => String::new(),
    };

    rsx! {
        span { class: style::wrapper,
            {children}
            if show {
                span { class: stylance::classes!(style::badge, size_class, style::positioned, dot_class),
                    if !is_dot {
                        "{content}"
                    }
                }
            }
        }
    }
}
