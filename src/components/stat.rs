//! Stat component for displaying statistics and metrics

use dioxus::prelude::*;

stylance::import_style!(style, "stat.module.scss");

/// Trend direction for stat changes
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StatTrend {
    /// No trend / neutral (default)
    #[default]
    Neutral,
    /// Upward trend (positive)
    Up,
    /// Downward trend (negative)
    Down,
}

/// Size variants for stat component
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StatSize {
    /// Small stat
    Small,
    /// Medium stat (default)
    #[default]
    Medium,
    /// Large stat
    Large,
}

/// A statistic display component showing a value with optional label and trend
#[component]
pub fn Stat(
    /// The main value to display
    #[props(into)]
    value: String,
    /// Label describing the stat
    #[props(optional, into)]
    label: Option<String>,
    /// Description or help text
    #[props(optional, into)]
    description: Option<String>,
    /// Change value (e.g., "+12%", "-5")
    #[props(optional, into)]
    change: Option<String>,
    /// Trend direction for styling
    #[props(default)]
    trend: StatTrend,
    /// Size variant
    #[props(default)]
    size: StatSize,
    /// Optional icon element
    #[props(optional)]
    icon: Option<Element>,
) -> Element {
    let size_class = match size {
        StatSize::Small => style::small,
        StatSize::Medium => "",
        StatSize::Large => style::large,
    };

    let trend_class = match trend {
        StatTrend::Neutral => "",
        StatTrend::Up => style::trend_up,
        StatTrend::Down => style::trend_down,
    };

    rsx! {
        div { class: stylance::classes!(style::stat, size_class),
            if let Some(icon_el) = icon {
                div { class: style::icon,
                    {icon_el}
                }
            }

            div { class: style::content,
                if let Some(lbl) = &label {
                    span { class: style::label, "{lbl}" }
                }

                span { class: style::value, "{value}" }

                if change.is_some() || description.is_some() {
                    div { class: style::footer,
                        if let Some(chg) = &change {
                            span { class: stylance::classes!(style::change, trend_class),
                                // Trend arrow
                                if trend != StatTrend::Neutral {
                                    span { class: style::trend_icon,
                                        svg {
                                            width: "12",
                                            height: "12",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            if trend == StatTrend::Up {
                                                polyline { points: "18 15 12 9 6 15" }
                                            } else {
                                                polyline { points: "6 9 12 15 18 9" }
                                            }
                                        }
                                    }
                                }
                                "{chg}"
                            }
                        }
                        if let Some(desc) = &description {
                            span { class: style::description, "{desc}" }
                        }
                    }
                }
            }
        }
    }
}

/// A group of stats displayed together
#[component]
pub fn StatGroup(
    /// The stats to display
    children: Element,
    /// Number of columns (1-4)
    #[props(default = 3)]
    columns: usize,
    /// Whether to show dividers between stats
    #[props(default = true)]
    dividers: bool,
) -> Element {
    let cols = columns.clamp(1, 4);

    rsx! {
        div {
            class: stylance::classes!(
                style::stat_group,
                if dividers { style::with_dividers } else { "" }
            ),
            style: format!("--stat-columns: {cols};"),
            {children}
        }
    }
}

/// A stat card with background styling
#[component]
pub fn StatCard(
    /// The main value to display
    #[props(into)]
    value: String,
    /// Label describing the stat
    #[props(optional, into)]
    label: Option<String>,
    /// Description or help text
    #[props(optional, into)]
    description: Option<String>,
    /// Change value
    #[props(optional, into)]
    change: Option<String>,
    /// Trend direction
    #[props(default)]
    trend: StatTrend,
    /// Size variant
    #[props(default)]
    size: StatSize,
    /// Optional icon
    #[props(optional)]
    icon: Option<Element>,
) -> Element {
    rsx! {
        div { class: style::stat_card,
            Stat {
                value,
                label,
                description,
                change,
                trend,
                size,
                icon,
            }
        }
    }
}
