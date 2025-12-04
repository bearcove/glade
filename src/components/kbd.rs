//! Keyboard shortcut display component

use dioxus::prelude::*;

stylance::import_style!(style, "kbd.module.scss");

/// Size variants for the Kbd component
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum KbdSize {
    /// Small keyboard key
    Sm,
    /// Medium keyboard key (default)
    #[default]
    Md,
    /// Large keyboard key
    Lg,
}

/// Keyboard key display component.
///
/// Renders a styled keyboard key, useful for displaying keyboard shortcuts.
#[component]
pub fn Kbd(
    /// Size of the keyboard key
    #[props(default)]
    size: KbdSize,
    /// Key text
    children: Element,
) -> Element {
    let size_class = match size {
        KbdSize::Sm => style::sm,
        KbdSize::Md => style::md,
        KbdSize::Lg => style::lg,
    };

    rsx! {
        kbd { class: stylance::classes!(style::kbd, size_class),
            {children}
        }
    }
}

/// A group of keyboard keys displayed together
#[component]
pub fn KbdGroup(children: Element) -> Element {
    rsx! {
        span { class: style::kbd_group,
            {children}
        }
    }
}
