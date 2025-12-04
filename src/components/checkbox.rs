//! Checkbox input component

use dioxus::prelude::*;

stylance::import_style!(style, "checkbox.module.scss");

/// Size variants for Checkbox
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CheckboxSize {
    /// Small checkbox
    Small,
    /// Medium checkbox (default)
    #[default]
    Medium,
    /// Large checkbox
    Large,
}

#[component]
pub fn Checkbox(
    #[props(default)] size: CheckboxSize,
    #[props(default = false)] checked: bool,
    #[props(default)] label: String,
    #[props(default = false)] disabled: bool,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let size_class = match size {
        CheckboxSize::Small => style::small,
        CheckboxSize::Medium => "",
        CheckboxSize::Large => style::large,
    };

    let disabled_class = if disabled { style::disabled } else { "" };

    rsx! {
        label { class: stylance::classes!(style::wrapper, disabled_class),
            input {
                class: stylance::classes!(style::checkbox, size_class),
                r#type: "checkbox",
                checked,
                disabled,
                onchange: move |evt| {
                    if let Some(handler) = &onchange {
                        handler.call(evt);
                    }
                },
            }
            span { class: style::checkmark }
            if !label.is_empty() {
                span { class: style::label, "{label}" }
            }
        }
    }
}
