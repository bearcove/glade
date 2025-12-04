//! Button component with variants, sizes, and loading state

use dioxus::prelude::*;

use super::spinner::{Spinner, SpinnerSize};

stylance::import_style!(style, "button.module.scss");

/// Visual style variants for Button
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    /// Primary action button (filled, accent color)
    Primary,
    /// Secondary action button (outlined)
    Secondary,
    /// Danger/destructive action button (red)
    Danger,
    /// Ghost button (transparent background)
    Ghost,
}

/// Size variants for Button
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    /// Small button
    Small,
    /// Medium button (default)
    Medium,
    /// Large button
    Large,
}

#[component]
pub fn Button(
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[props(default = ButtonSize::Medium)] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] loading: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => style::primary,
        ButtonVariant::Secondary => style::secondary,
        ButtonVariant::Danger => style::danger,
        ButtonVariant::Ghost => style::ghost,
    };

    let size_class = match size {
        ButtonSize::Small => style::small,
        ButtonSize::Medium => "",
        ButtonSize::Large => style::large,
    };

    let loading_class = if loading { style::loading } else { "" };

    // Map button size to spinner size
    let spinner_size = match size {
        ButtonSize::Small => SpinnerSize::Small,
        ButtonSize::Medium => SpinnerSize::Small,
        ButtonSize::Large => SpinnerSize::Medium,
    };

    rsx! {
        button {
            class: stylance::classes!(style::button, variant_class, size_class, loading_class),
            disabled: disabled || loading,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            span { class: style::content, {children} }
            if loading {
                span { class: style::spinner_wrapper,
                    Spinner { size: spinner_size }
                }
            }
        }
    }
}
