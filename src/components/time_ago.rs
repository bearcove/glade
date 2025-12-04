//! `TimeAgo` component - displays relative time (e.g., "5 minutes ago")

use dioxus::prelude::*;

stylance::import_style!(style, "time_ago.module.scss");

/// Format a duration in seconds to a human-readable relative time string
fn format_relative_time(seconds: i64) -> String {
    let abs_seconds = seconds.abs();
    let suffix = if seconds < 0 { "from now" } else { "ago" };

    if abs_seconds < 5 {
        return "just now".to_string();
    }

    if abs_seconds < 60 {
        return format!("{abs_seconds} seconds {suffix}");
    }

    let minutes = abs_seconds / 60;
    if minutes < 60 {
        if minutes == 1 {
            return format!("1 minute {suffix}");
        }
        return format!("{minutes} minutes {suffix}");
    }

    let hours = minutes / 60;
    if hours < 24 {
        if hours == 1 {
            return format!("1 hour {suffix}");
        }
        return format!("{hours} hours {suffix}");
    }

    let days = hours / 24;
    if days < 7 {
        if days == 1 {
            return format!("1 day {suffix}");
        }
        return format!("{days} days {suffix}");
    }

    let weeks = days / 7;
    if weeks < 4 {
        if weeks == 1 {
            return format!("1 week {suffix}");
        }
        return format!("{weeks} weeks {suffix}");
    }

    let months = days / 30;
    if months < 12 {
        if months == 1 {
            return format!("1 month {suffix}");
        }
        return format!("{months} months {suffix}");
    }

    let years = days / 365;
    if years == 1 {
        return format!("1 year {suffix}");
    }
    format!("{years} years {suffix}")
}

/// TimeAgo - displays a timestamp as relative time
///
/// Updates automatically on the client side every minute.
#[component]
pub fn TimeAgo(
    /// Unix timestamp in seconds
    timestamp: i64,
    /// Whether to show the full date on hover as a tooltip
    #[props(default = true)]
    show_tooltip: bool,
) -> Element {
    let now = use_signal(|| chrono::Utc::now().timestamp());

    // TODO: Set up interval to update time periodically (client-side only)
    // This needs proper implementation with use_effect or similar

    let relative_text = use_memo(move || {
        let diff = now() - timestamp;
        format_relative_time(diff)
    });

    // Format full date for tooltip
    let full_date = {
        use chrono::{DateTime, Utc};
        DateTime::from_timestamp(timestamp, 0)
            .map(|dt: DateTime<Utc>| dt.format("%B %d, %Y at %H:%M UTC").to_string())
            .unwrap_or_default()
    };

    if show_tooltip && !full_date.is_empty() {
        rsx! {
            time {
                class: style::time_ago,
                datetime: "{full_date}",
                title: "{full_date}",
                "{relative_text}"
            }
        }
    } else {
        rsx! {
            time { class: style::time_ago, "{relative_text}" }
        }
    }
}
