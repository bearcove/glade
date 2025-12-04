//! IconButton - a minimal button for icon-only actions

use dioxus::prelude::*;

stylance::import_style!(style, "icon_button.module.scss");

/// Size variants for IconButton
#[derive(Clone, Copy, PartialEq, Default)]
pub enum IconButtonSize {
    /// Small icon button (24px)
    Small,
    /// Medium icon button (32px, default)
    #[default]
    Medium,
    /// Large icon button (40px)
    Large,
}

/// Visual style variants for IconButton
#[derive(Clone, Copy, PartialEq, Default)]
pub enum IconButtonVariant {
    /// Ghost button (transparent, no background)
    #[default]
    Ghost,
    /// Subtle button (light background on hover)
    Subtle,
}

/// IconButton - a square button for icon-only actions
///
/// Shows no background by default, subtle background on hover.
/// Used for close buttons, toolbar actions, etc.
#[component]
pub fn IconButton(
    #[props(default)] size: IconButtonSize,
    #[props(default)] variant: IconButtonVariant,
    #[props(default = false)] disabled: bool,
    #[props(default)] aria_label: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let size_class = match size {
        IconButtonSize::Small => style::small,
        IconButtonSize::Medium => "",
        IconButtonSize::Large => style::large,
    };

    let variant_class = match variant {
        IconButtonVariant::Ghost => style::ghost,
        IconButtonVariant::Subtle => style::subtle,
    };

    rsx! {
        button {
            class: stylance::classes!(style::icon_button, size_class, variant_class),
            disabled,
            "aria-label": "{aria_label}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}
