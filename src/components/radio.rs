//! Radio button input component

use dioxus::prelude::*;

stylance::import_style!(style, "radio.module.scss");

/// Size variants for Radio
#[derive(Clone, Copy, PartialEq, Default)]
pub enum RadioSize {
    /// Small radio button
    Small,
    /// Medium radio button (default)
    #[default]
    Medium,
    /// Large radio button
    Large,
}

#[component]
pub fn Radio(
    #[props(default)] size: RadioSize,
    #[props(default = false)] checked: bool,
    #[props(default)] label: String,
    #[props(default)] name: String,
    #[props(default)] value: String,
    #[props(default = false)] disabled: bool,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let size_class = match size {
        RadioSize::Small => style::small,
        RadioSize::Medium => "",
        RadioSize::Large => style::large,
    };

    let disabled_class = if disabled { style::disabled } else { "" };

    rsx! {
        label { class: stylance::classes!(style::wrapper, disabled_class),
            input {
                class: stylance::classes!(style::radio, size_class),
                r#type: "radio",
                name,
                value,
                checked,
                disabled,
                onchange: move |evt| {
                    if let Some(handler) = &onchange {
                        handler.call(evt);
                    }
                },
            }
            span { class: style::circle }
            if !label.is_empty() {
                span { class: style::label, "{label}" }
            }
        }
    }
}

#[component]
pub fn RadioGroup(#[props(default)] label: String, children: Element) -> Element {
    rsx! {
        fieldset { class: style::group,
            if !label.is_empty() {
                legend { class: style::group_label, "{label}" }
            }
            div { class: style::group_options,
                {children}
            }
        }
    }
}
