//! Alert component for displaying contextual feedback messages

use dioxus::prelude::*;

use super::icon_button::{IconButton, IconButtonSize};
use super::icons::IconX;

stylance::import_style!(#[allow(dead_code)] style, "alert.module.scss");

/// Alert style variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum AlertVariant {
    /// Informational message (default)
    #[default]
    Info,
    /// Success/confirmation message
    Success,
    /// Warning/caution message
    Warning,
    /// Error/critical message
    Error,
}

#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(default)] title: String,
    #[props(default = false)] dismissible: bool,
    ondismiss: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        AlertVariant::Info => style::info,
        AlertVariant::Success => style::success,
        AlertVariant::Warning => style::warning,
        AlertVariant::Error => style::error,
    };

    rsx! {
        div { class: stylance::classes!(style::alert, variant_class), role: "alert",
            div { class: style::content,
                if !title.is_empty() {
                    div { class: style::title, "{title}" }
                }
                div { class: style::message,
                    {children}
                }
            }
            if dismissible {
                IconButton {
                    size: IconButtonSize::Small,
                    aria_label: "Dismiss".to_string(),
                    onclick: move |evt| {
                        if let Some(handler) = &ondismiss {
                            handler.call(evt);
                        }
                    },
                    IconX {}
                }
            }
        }
    }
}
