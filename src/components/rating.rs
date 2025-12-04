//! Rating component for star ratings and reviews

use dioxus::prelude::*;

stylance::import_style!(style, "rating.module.scss");

/// Size variants for the rating component
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum RatingSize {
    /// Small rating stars
    Small,
    /// Medium rating stars (default)
    #[default]
    Medium,
    /// Large rating stars
    Large,
}

/// Star rating input component
#[component]
pub fn Rating(
    /// Current rating value (0 to max)
    #[props(default = 0.0)]
    value: f64,
    /// Maximum rating (number of stars)
    #[props(default = 5)]
    max: usize,
    /// Callback when rating changes
    #[props(optional)]
    on_change: Option<EventHandler<f64>>,
    /// Size variant
    #[props(default)]
    size: RatingSize,
    /// Whether the rating is read-only
    #[props(default = false)]
    readonly: bool,
    /// Whether to allow half-star ratings
    #[props(default = false)]
    allow_half: bool,
    /// Whether the rating is disabled
    #[props(default = false)]
    disabled: bool,
    /// Aria label for accessibility
    #[props(optional, into)]
    aria_label: Option<String>,
) -> Element {
    let mut hover_value = use_signal(|| None::<f64>);

    let size_class = match size {
        RatingSize::Small => style::small,
        RatingSize::Medium => "",
        RatingSize::Large => style::large,
    };

    let display_value = hover_value().unwrap_or(value);
    let is_interactive = !readonly && !disabled && on_change.is_some();

    rsx! {
        div {
            class: stylance::classes!(
                style::rating,
                size_class,
                if readonly { style::readonly } else { "" },
                if disabled { style::disabled } else { "" }
            ),
            role: "slider",
            aria_label: aria_label.unwrap_or_else(|| "Rating".to_string()),
            aria_valuenow: value.to_string(),
            aria_valuemin: "0",
            aria_valuemax: max.to_string(),
            onmouseleave: move |_| {
                if is_interactive {
                    hover_value.set(None);
                }
            },

            for i in 0..max {
                {
                    let star_value = (i + 1) as f64;
                    let half_value = star_value - 0.5;

                    // Determine fill state
                    let fill = if display_value >= star_value {
                        StarFill::Full
                    } else if display_value >= half_value {
                        StarFill::Half
                    } else {
                        StarFill::Empty
                    };

                    rsx! {
                        button {
                            key: "{i}",
                            class: stylance::classes!(
                                style::star,
                                match fill {
                                    StarFill::Full => style::filled,
                                    StarFill::Half => style::half,
                                    StarFill::Empty => "",
                                }
                            ),
                            disabled: !is_interactive,
                            tabindex: if is_interactive { "0" } else { "-1" },
                            onmouseenter: move |_| {
                                if is_interactive {
                                    hover_value.set(Some(star_value));
                                }
                            },
                            onclick: move |_| {
                                if let Some(handler) = &on_change {
                                    if !readonly && !disabled {
                                        handler.call(star_value);
                                    }
                                }
                            },
                            // For half-star support on click position
                            onmousemove: move |e| {
                                if is_interactive && allow_half {
                                    let rect = e.data().element_coordinates();
                                    // If mouse is in left half of star, show half rating
                                    if rect.x < 12.0 { // Approximate half of star width
                                        hover_value.set(Some(half_value));
                                    } else {
                                        hover_value.set(Some(star_value));
                                    }
                                }
                            },
                            aria_label: format!("{} star{}", star_value, if star_value == 1.0 { "" } else { "s" }),

                            // Star SVG
                            svg {
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                stroke: "currentColor",
                                stroke_width: "1",
                                // Filled star path
                                if matches!(fill, StarFill::Full) {
                                    path {
                                        d: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"
                                    }
                                }
                                // Half star (using clip path)
                                if matches!(fill, StarFill::Half) {
                                    defs {
                                        clipPath { id: "half-{i}",
                                            rect { x: "0", y: "0", width: "12", height: "24" }
                                        }
                                    }
                                    // Empty outline
                                    path {
                                        d: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z",
                                        fill: "none",
                                        stroke_width: "2"
                                    }
                                    // Filled half
                                    path {
                                        d: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z",
                                        clip_path: "url(#half-{i})"
                                    }
                                }
                                // Empty star
                                if matches!(fill, StarFill::Empty) {
                                    path {
                                        d: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z",
                                        fill: "none",
                                        stroke_width: "2"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum StarFill {
    Empty,
    Half,
    Full,
}

/// Display-only rating with numeric value
#[component]
pub fn RatingDisplay(
    /// Rating value
    value: f64,
    /// Maximum rating
    #[props(default = 5)]
    max: usize,
    /// Size variant
    #[props(default)]
    size: RatingSize,
    /// Whether to show the numeric value
    #[props(default = true)]
    show_value: bool,
    /// Number of reviews (optional)
    #[props(optional)]
    reviews: Option<usize>,
) -> Element {
    rsx! {
        div { class: style::rating_display,
            Rating {
                value,
                max,
                size,
                readonly: true,
            }
            if show_value {
                span { class: style::value, "{value:.1}" }
            }
            if let Some(count) = reviews {
                span { class: style::reviews, "({count})" }
            }
        }
    }
}
