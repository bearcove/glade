//! Select dropdown component

use dioxus::prelude::*;

stylance::import_style!(style, "select.module.scss");

/// Size variants for Select
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SelectSize {
    /// Small select
    Small,
    /// Medium select (default)
    #[default]
    Medium,
    /// Large select
    Large,
}

#[component]
pub fn Select(
    #[props(default)] size: SelectSize,
    #[props(default)] value: String,
    #[props(default)] label: String,
    #[props(default)] error: String,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] required: bool,
    onchange: Option<EventHandler<FormEvent>>,
    children: Element,
) -> Element {
    let size_class = match size {
        SelectSize::Small => style::small,
        SelectSize::Medium => "",
        SelectSize::Large => style::large,
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
            div { class: style::container,
                select {
                    class: stylance::classes!(style::select, size_class, error_class),
                    value,
                    disabled,
                    required,
                    onchange: move |evt| {
                        if let Some(handler) = &onchange {
                            handler.call(evt);
                        }
                    },
                    {children}
                }
                span { class: style::arrow }
            }
            if !error.is_empty() {
                span { class: style::error_text, "{error}" }
            }
        }
    }
}
