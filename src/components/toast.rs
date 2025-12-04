//! Toast/Snackbar notification system

use dioxus::prelude::*;

use crate::{IconAlertCircle, IconCheck, IconInfo, IconX};

stylance::import_style!(style, "toast.module.scss");

/// Toast variant/type
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    /// Informational toast
    #[default]
    Info,
    /// Success toast
    Success,
    /// Warning toast
    Warning,
    /// Error toast
    Error,
}

/// Position for the toast container
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    /// Top right corner
    #[default]
    TopRight,
    /// Top left corner
    TopLeft,
    /// Bottom right corner
    BottomRight,
    /// Bottom left corner
    BottomLeft,
    /// Top center
    TopCenter,
    /// Bottom center
    BottomCenter,
}

/// A single toast notification
#[component]
pub fn Toast(
    /// Toast variant
    #[props(default = ToastVariant::Info)]
    variant: ToastVariant,
    /// Toast title
    #[props(optional)]
    title: Option<String>,
    /// Whether the toast can be dismissed
    #[props(default = true)]
    dismissible: bool,
    /// Called when dismissed
    #[props(optional)]
    on_dismiss: Option<EventHandler<()>>,
    /// Toast content
    children: Element,
) -> Element {
    let variant_class = match variant {
        ToastVariant::Info => style::info,
        ToastVariant::Success => style::success,
        ToastVariant::Warning => style::warning,
        ToastVariant::Error => style::error,
    };

    let icon = match variant {
        ToastVariant::Info => rsx! { IconInfo {} },
        ToastVariant::Success => rsx! { IconCheck {} },
        ToastVariant::Warning => rsx! { IconAlertCircle {} },
        ToastVariant::Error => rsx! { IconAlertCircle {} },
    };

    rsx! {
        div { class: stylance::classes!(style::toast, variant_class),
            div { class: style::icon, {icon} }
            div { class: style::content,
                {title.as_ref().map(|t| rsx! { div { class: style::title, {t.clone()} } })}
                div { class: style::message, {children} }
            }
            if dismissible && on_dismiss.is_some() {
                button {
                    r#type: "button",
                    class: style::dismiss,
                    onclick: move |_| {
                        if let Some(handler) = &on_dismiss {
                            handler.call(());
                        }
                    },
                    IconX {}
                }
            }
        }
    }
}

/// Container for positioning toasts
#[component]
pub fn ToastContainer(
    /// Position of the container
    #[props(default = ToastPosition::TopRight)]
    position: ToastPosition,
    /// Toast elements
    children: Element,
) -> Element {
    let position_class = match position {
        ToastPosition::TopRight => style::top_right,
        ToastPosition::TopLeft => style::top_left,
        ToastPosition::BottomRight => style::bottom_right,
        ToastPosition::BottomLeft => style::bottom_left,
        ToastPosition::TopCenter => style::top_center,
        ToastPosition::BottomCenter => style::bottom_center,
    };

    rsx! { div { class: stylance::classes!(style::container, position_class), {children} } }
}
