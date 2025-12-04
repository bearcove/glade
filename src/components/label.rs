//! Label component for form fields with proper accessibility

use dioxus::prelude::*;

stylance::import_style!(style, "label.module.scss");

/// Size variants for labels
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelSize {
    /// Small label text
    Small,
    /// Medium label text (default)
    #[default]
    Medium,
    /// Large label text
    Large,
}

/// Label component for form fields
#[component]
pub fn Label(
    /// The id of the form element this label is for
    #[props(optional, into)]
    html_for: Option<String>,
    /// Label text content
    children: Element,
    /// Size variant
    #[props(default)]
    size: LabelSize,
    /// Whether the field is required (shows indicator)
    #[props(default = false)]
    required: bool,
    /// Whether the field is optional (shows "(optional)" text)
    #[props(default = false)]
    optional: bool,
    /// Whether the label is disabled
    #[props(default = false)]
    disabled: bool,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let size_class = match size {
        LabelSize::Small => style::small,
        LabelSize::Medium => "",
        LabelSize::Large => style::large,
    };

    rsx! {
        label {
            class: stylance::classes!(
                style::label,
                size_class,
                if disabled { style::disabled } else { "" },
                class.as_deref().unwrap_or("")
            ),
            r#for: html_for,

            span { class: style::text, {children} }

            if required {
                span {
                    class: style::required,
                    aria_hidden: "true",
                    "*"
                }
            }

            if optional {
                span { class: style::optional_text, "(optional)" }
            }
        }
    }
}

/// A form field wrapper that includes label, input slot, helper text, and error message
#[component]
pub fn FormField(
    /// Label text
    #[props(into)]
    label: String,
    /// Unique id for the input (used for label association)
    #[props(into)]
    id: String,
    /// The form input element
    children: Element,
    /// Helper text shown below the input
    #[props(optional, into)]
    helper: Option<String>,
    /// Error message (replaces helper when present)
    #[props(optional, into)]
    error: Option<String>,
    /// Whether the field is required
    #[props(default = false)]
    required: bool,
    /// Whether to show "(optional)" instead of required indicator
    #[props(default = false)]
    optional: bool,
    /// Size variant
    #[props(default)]
    size: LabelSize,
    /// Whether the field is disabled
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let has_error = error.is_some();

    rsx! {
        div {
            class: stylance::classes!(
                style::form_field,
                if has_error { style::has_error } else { "" },
                if disabled { style::disabled } else { "" }
            ),

            Label {
                html_for: id.clone(),
                size,
                required,
                optional,
                disabled,
                "{label}"
            }

            div { class: style::input_wrapper,
                {children}
            }

            if let Some(err) = error {
                p {
                    class: style::error_message,
                    id: format!("{id}-error"),
                    role: "alert",
                    "{err}"
                }
            } else if let Some(help) = helper {
                p {
                    class: style::helper_text,
                    id: format!("{id}-helper"),
                    "{help}"
                }
            }
        }
    }
}
