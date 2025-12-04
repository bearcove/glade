//! Steps component for multi-step processes and wizards

use dioxus::prelude::*;

stylance::import_style!(style, "steps.module.scss");

/// Status of a step
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    /// Step not yet reached (default)
    #[default]
    Pending,
    /// Currently active step
    Current,
    /// Step completed successfully
    Completed,
    /// Step encountered an error
    Error,
}

/// Size variants for steps
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StepsSize {
    /// Small step indicators
    Small,
    /// Medium step indicators (default)
    #[default]
    Medium,
    /// Large step indicators
    Large,
}

/// Orientation of the steps
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StepsOrientation {
    /// Steps arranged horizontally (default)
    #[default]
    Horizontal,
    /// Steps arranged vertically
    Vertical,
}

/// Steps container component
#[component]
pub fn Steps(
    /// Current step index (0-based)
    #[props(default = 0)]
    current: usize,
    /// Size variant
    #[props(default)]
    size: StepsSize,
    /// Orientation
    #[props(default)]
    orientation: StepsOrientation,
    /// Step items
    children: Element,
) -> Element {
    let size_class = match size {
        StepsSize::Small => style::small,
        StepsSize::Medium => "",
        StepsSize::Large => style::large,
    };

    let orientation_class = match orientation {
        StepsOrientation::Horizontal => style::horizontal,
        StepsOrientation::Vertical => style::vertical,
    };

    // Provide current step to children
    use_context_provider(|| StepsContext { current });

    rsx! {
        div {
            class: stylance::classes!(style::steps, size_class, orientation_class),
            role: "list",
            aria_label: "Progress steps",
            {children}
        }
    }
}

#[derive(Clone, Copy)]
struct StepsContext {
    current: usize,
}

/// Individual step item
#[component]
pub fn Step(
    /// Step index (0-based)
    index: usize,
    /// Step title
    #[props(into)]
    title: String,
    /// Optional description
    #[props(optional, into)]
    description: Option<String>,
    /// Optional custom icon
    #[props(optional)]
    icon: Option<Element>,
    /// Override status (otherwise computed from current)
    #[props(optional)]
    status: Option<StepStatus>,
    /// Whether this is the last step (no connector after)
    #[props(default = false)]
    last: bool,
) -> Element {
    let ctx = use_context::<StepsContext>();

    let computed_status = status.unwrap_or_else(|| {
        if index < ctx.current {
            StepStatus::Completed
        } else if index == ctx.current {
            StepStatus::Current
        } else {
            StepStatus::Pending
        }
    });

    let status_class = match computed_status {
        StepStatus::Pending => style::pending,
        StepStatus::Current => style::current,
        StepStatus::Completed => style::completed,
        StepStatus::Error => style::error,
    };

    rsx! {
        div {
            class: stylance::classes!(style::step, status_class),
            role: "listitem",
            aria_current: if computed_status == StepStatus::Current { "step" } else { "" },

            // Step indicator (number or icon)
            div { class: style::indicator,
                if let Some(custom_icon) = icon {
                    {custom_icon}
                } else {
                    match computed_status {
                        StepStatus::Completed => rsx! {
                            svg {
                                width: "16",
                                height: "16",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "3",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        },
                        StepStatus::Error => rsx! {
                            svg {
                                width: "16",
                                height: "16",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "3",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        },
                        _ => rsx! {
                            span { "{index + 1}" }
                        },
                    }
                }
            }

            // Step content
            div { class: style::content,
                span { class: style::title, "{title}" }
                if let Some(desc) = description {
                    span { class: style::description, "{desc}" }
                }
            }

            // Connector line (not for last step)
            if !last {
                div { class: style::connector }
            }
        }
    }
}

/// Clickable step item for navigation
#[component]
pub fn StepButton(
    /// Step index (0-based)
    index: usize,
    /// Step title
    #[props(into)]
    title: String,
    /// Optional description
    #[props(optional, into)]
    description: Option<String>,
    /// Click handler
    onclick: EventHandler<usize>,
    /// Override status
    #[props(optional)]
    status: Option<StepStatus>,
    /// Whether this is the last step
    #[props(default = false)]
    last: bool,
    /// Whether clicking is disabled
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let ctx = use_context::<StepsContext>();

    let computed_status = status.unwrap_or_else(|| {
        if index < ctx.current {
            StepStatus::Completed
        } else if index == ctx.current {
            StepStatus::Current
        } else {
            StepStatus::Pending
        }
    });

    let status_class = match computed_status {
        StepStatus::Pending => style::pending,
        StepStatus::Current => style::current,
        StepStatus::Completed => style::completed,
        StepStatus::Error => style::error,
    };

    rsx! {
        button {
            class: stylance::classes!(style::step, style::clickable, status_class),
            disabled,
            onclick: move |_| onclick.call(index),
            role: "listitem",
            aria_current: if computed_status == StepStatus::Current { "step" } else { "" },

            div { class: style::indicator,
                match computed_status {
                    StepStatus::Completed => rsx! {
                        svg {
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                    },
                    StepStatus::Error => rsx! {
                        svg {
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    },
                    _ => rsx! {
                        span { "{index + 1}" }
                    },
                }
            }

            div { class: style::content,
                span { class: style::title, "{title}" }
                if let Some(desc) = description {
                    span { class: style::description, "{desc}" }
                }
            }

            if !last {
                div { class: style::connector }
            }
        }
    }
}
