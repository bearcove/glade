//! Textarea component for multi-line text input

use dioxus::prelude::*;

stylance::import_style!(style, "textarea.module.scss");

/// Size variants for Textarea
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TextareaSize {
    /// Small textarea
    Small,
    /// Medium textarea (default)
    #[default]
    Medium,
    /// Large textarea
    Large,
}

#[component]
pub fn Textarea(
    #[props(default)] size: TextareaSize,
    #[props(default)] placeholder: String,
    #[props(default)] value: String,
    #[props(default)] label: String,
    #[props(default)] error: String,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] required: bool,
    #[props(default = 4)] rows: u32,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let size_class = match size {
        TextareaSize::Small => style::small,
        TextareaSize::Medium => "",
        TextareaSize::Large => style::large,
    };

    let error_class = if !error.is_empty() { style::error } else { "" };

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
            textarea {
                class: stylance::classes!(style::textarea, size_class, error_class),
                placeholder,
                value,
                disabled,
                required,
                rows: rows as i64,
                oninput: move |evt| {
                    if let Some(handler) = &oninput {
                        handler.call(evt);
                    }
                },
            }
            if !error.is_empty() {
                span { class: style::error_text, "{error}" }
            }
        }
    }
}
