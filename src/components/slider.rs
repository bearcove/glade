//! Slider/range input component

use dioxus::prelude::*;

stylance::import_style!(style, "slider.module.scss");

/// Size variants for the Slider
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderSize {
    /// Small slider
    Small,
    /// Medium slider (default)
    #[default]
    Medium,
    /// Large slider
    Large,
}

/// Slider/range input component
#[component]
pub fn Slider(
    /// Current value
    value: Signal<f64>,
    /// Callback when value changes
    #[props(optional)]
    on_change: Option<EventHandler<f64>>,
    /// Minimum value (default: 0)
    #[props(default = 0.0)]
    min: f64,
    /// Maximum value (default: 100)
    #[props(default = 100.0)]
    max: f64,
    /// Step increment (default: 1)
    #[props(default = 1.0)]
    step: f64,
    /// Size variant
    #[props(default = SliderSize::Medium)]
    size: SliderSize,
    /// Whether the slider is disabled
    #[props(default = false)]
    disabled: bool,
    /// Whether to show the current value label
    #[props(default = false)]
    show_value: bool,
    /// Label text
    #[props(optional, into)]
    label: Option<String>,
    /// Custom value formatter (takes f64, returns String via callback)
    #[props(optional)]
    format_value: Option<Callback<f64, String>>,
) -> Element {
    let size_class = match size {
        SliderSize::Small => style::small,
        SliderSize::Medium => "",
        SliderSize::Large => style::large,
    };

    let percentage = move || ((value() - min) / (max - min) * 100.0).clamp(0.0, 100.0);

    let formatted_value = move || {
        if let Some(formatter) = &format_value {
            formatter.call(value())
        } else {
            format!("{}", value())
        }
    };

    let handle_input = move |evt: FormEvent| {
        if let Ok(new_value) = evt.value().parse::<f64>() {
            if let Some(cb) = &on_change {
                cb.call(new_value);
            }
        }
    };

    rsx! {
        div { class: stylance::classes!(style::slider_wrapper, size_class),
            if let Some(l) = &label {
                div { class: style::header,
                    label { class: style::label, {l.clone()} }
                    if show_value {
                        span { class: style::value, {formatted_value()} }
                    }
                }
            }

            if label.is_none() && show_value {
                span { class: style::value_standalone, {formatted_value()} }
            }

            div { class: style::track_container,
                input {
                    r#type: "range",
                    class: style::input,
                    min: min,
                    max: max,
                    step: step,
                    disabled: disabled,
                    value: value(),
                    oninput: handle_input,
                    style: format!("--slider-progress: {}%", percentage()),
                }
            }
        }
    }
}
