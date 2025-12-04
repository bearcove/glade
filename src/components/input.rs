//! Text input component

use dioxus::prelude::*;

use crate::IconX;

stylance::import_style!(style, "input.module.scss");

/// Size variants for Input
#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputSize {
    /// Small input
    Small,
    /// Medium input (default)
    #[default]
    Medium,
    /// Large input
    Large,
}

#[component]
pub fn Input(
    #[props(default)] size: InputSize,
    #[props(default = "text".to_string())] r#type: String,
    #[props(default)] placeholder: String,
    #[props(default)] value: String,
    #[props(default)] label: String,
    #[props(default)] error: String,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] required: bool,
    /// Show a clear button when input has content
    #[props(default = false)] clearable: bool,
    /// Icon to show at the start of the input
    icon: Option<Element>,
    oninput: Option<EventHandler<FormEvent>>,
    /// Called when the clear button is clicked
    onclear: Option<EventHandler<()>>,
) -> Element {
    let size_class = match size {
        InputSize::Small => style::small,
        InputSize::Medium => "",
        InputSize::Large => style::large,
    };

    let error_class = if !error.is_empty() { style::error } else { "" };
    let has_value = !value.is_empty();
    let show_clear = clearable && has_value;
    let has_icon = icon.is_some();

    rsx! {
        div { class: style::wrapper,
            if !label.is_empty() {
                label { class: style::label,
                    "{label}"
                    if required {
                        span { class: style::required, " *" }
                    }
                }
            }
            div { class: style::input_container,
                if let Some(icon_el) = icon {
                    span { class: style::icon, {icon_el} }
                }
                input {
                    class: stylance::classes!(style::input, size_class, error_class, if clearable { style::has_clear } else { "" }, if has_icon { style::has_icon } else { "" }),
                    r#type,
                    placeholder,
                    value,
                    disabled,
                    required,
                    oninput: move |evt| {
                        if let Some(handler) = &oninput {
                            handler.call(evt);
                        }
                    },
                }
                if clearable {
                    button {
                        r#type: "button",
                        class: stylance::classes!(style::clear_button, if show_clear { "" } else { style::clear_hidden }),
                        disabled,
                        onclick: move |_| {
                            if let Some(handler) = &onclear {
                                handler.call(());
                            }
                        },
                        IconX {}
                    }
                }
            }
            if !error.is_empty() {
                span { class: style::error_text, "{error}" }
            }
        }
    }
}
