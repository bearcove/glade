//! Toggle component for on/off states

use dioxus::prelude::*;

stylance::import_style!(style, "toggle.module.scss");

/// Size variants for Toggle
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ToggleSize {
    /// Small toggle
    Small,
    /// Medium toggle (default)
    #[default]
    Medium,
    /// Large toggle
    Large,
}

/// Toggle - a control for on/off states
#[component]
pub fn Toggle(
    #[props(default)] size: ToggleSize,
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default)] label: String,
    onchange: Option<EventHandler<bool>>,
) -> Element {
    let size_class = match size {
        ToggleSize::Small => style::small,
        ToggleSize::Medium => "",
        ToggleSize::Large => style::large,
    };

    let checked_class = if checked { style::checked } else { "" };
    let disabled_class = if disabled { style::disabled } else { "" };

    rsx! {
        label { class: stylance::classes!(style::wrapper, disabled_class),
            button {
                class: stylance::classes!(style::toggle, size_class, checked_class),
                role: "switch",
                "aria-checked": "{checked}",
                disabled,
                onclick: move |_| {
                    if let Some(handler) = &onchange {
                        handler.call(!checked);
                    }
                },
                span { class: style::thumb }
            }
            if !label.is_empty() {
                span { class: style::label, "{label}" }
            }
        }
    }
}
