//! Progress bars (linear and circular) for completion status

use dioxus::prelude::*;

stylance::import_style!(style, "progress.module.scss");

/// Size variants for the Progress bar
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressSize {
    /// Small progress bar (2px height)
    Small,
    /// Medium progress bar (default, 4px height)
    #[default]
    Medium,
    /// Large progress bar (8px height)
    Large,
}

/// Visual style variants for the Progress bar
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressVariant {
    /// Default/primary color
    #[default]
    Default,
    /// Success/green color
    Success,
    /// Warning/yellow color
    Warning,
    /// Error/red color
    Error,
}

/// Progress bar component for showing completion status
#[component]
pub fn Progress(
    #[props(into)] value: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = ProgressSize::Medium)] size: ProgressSize,
    #[props(default = ProgressVariant::Default)] variant: ProgressVariant,
    #[props(default = false)] show_label: bool,
    #[props(optional)] label_format: Option<Callback<f64, String>>,
    #[props(default = false)] animated: bool,
    #[props(default = false)] indeterminate: bool,
    #[props(optional, into)] aria_label: Option<String>,
) -> Element {
    let size_class = match size {
        ProgressSize::Small => style::small,
        ProgressSize::Medium => "",
        ProgressSize::Large => style::large,
    };

    let variant_class = match variant {
        ProgressVariant::Default => "",
        ProgressVariant::Success => style::success,
        ProgressVariant::Warning => style::warning,
        ProgressVariant::Error => style::error,
    };

    let percentage = move || ((value / max) * 100.0).clamp(0.0, 100.0);

    let label_text = move || {
        let pct = percentage();
        if let Some(format_fn) = &label_format {
            format_fn.call(pct)
        } else {
            format!("{pct:.0}%")
        }
    };

    rsx! {
        div {
            class: stylance::classes!(style::progress_wrapper, size_class, variant_class),
            role: "progressbar",
            aria_valuenow: if indeterminate { String::new() } else { value.to_string() },
            aria_valuemin: "0",
            aria_valuemax: max.to_string(),
            aria_label: aria_label,
            div { class: style::track,
                div {
                    class: stylance::classes!(
                        style::bar,
                        if indeterminate { style::indeterminate } else { "" },
                        if animated { style::animated } else { "" }
                    ),
                    style: format!("width: {}%", if indeterminate { 100.0 } else { percentage() }),
                }
            }
            if show_label {
                span { class: style::label, {label_text()} }
            }
        }
    }
}

/// Circular progress indicator
#[component]
pub fn CircularProgress(
    #[props(optional, default = 0.0)] value: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 48)] size: u32,
    #[props(default = 4)] stroke_width: u32,
    #[props(default = ProgressVariant::Default)] variant: ProgressVariant,
    #[props(default = false)] show_label: bool,
    #[props(default = false)] indeterminate: bool,
) -> Element {
    let variant_class = match variant {
        ProgressVariant::Default => "",
        ProgressVariant::Success => style::success,
        ProgressVariant::Warning => style::warning,
        ProgressVariant::Error => style::error,
    };

    let radius = (size.saturating_sub(stroke_width)) / 2;
    let circumference = 2.0 * std::f64::consts::PI * (radius as f64);

    let percentage = move || ((value / max) * 100.0).clamp(0.0, 100.0);

    let stroke_dashoffset = move || {
        if indeterminate {
            circumference * 0.75
        } else {
            circumference - (percentage() / 100.0 * circumference)
        }
    };

    let center = size / 2;

    rsx! {
        div {
            class: stylance::classes!(
                style::circular_wrapper,
                variant_class,
                if indeterminate { style::circular_indeterminate } else { "" }
            ),
            style: format!("width: {}px; height: {}px;", size, size),
            role: "progressbar",
            aria_valuenow: if indeterminate { String::new() } else { value.to_string() },
            aria_valuemin: "0",
            aria_valuemax: max.to_string(),
            svg {
                width: size,
                height: size,
                view_box: format!("0 0 {} {}", size, size),
                class: style::circular_svg,
                circle {
                    class: style::circular_track,
                    cx: center,
                    cy: center,
                    r: radius,
                    stroke_width: stroke_width,
                    fill: "none",
                }
                circle {
                    class: style::circular_bar,
                    cx: center,
                    cy: center,
                    r: radius,
                    stroke_width: stroke_width,
                    fill: "none",
                    stroke_dasharray: circumference,
                    stroke_dashoffset: stroke_dashoffset(),
                    stroke_linecap: "round",
                    transform: format!("rotate(-90 {} {})", center, center),
                }
            }
            if show_label {
                span { class: style::circular_label, {format!("{:.0}%", percentage())} }
            }
        }
    }
}
