//! Tag input component for entering multiple values as chips/tags

use dioxus::prelude::*;

use crate::IconX;

stylance::import_style!(style, "tag_input.module.scss");

/// Size variants for `TagInput`
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TagInputSize {
    /// Small tag input
    Small,
    /// Medium tag input (default)
    #[default]
    Medium,
    /// Large tag input
    Large,
}

/// A single tag/chip with remove button
#[component]
pub fn Tag(
    /// The tag text to display
    label: String,
    /// Called when the remove button is clicked
    #[props(optional)]
    on_remove: Option<EventHandler<()>>,
    /// Whether the tag can be removed
    #[props(default = true)]
    removable: bool,
) -> Element {
    rsx! {
        span { class: style::tag,
            span { class: style::tag_label, {label} }
            if removable && on_remove.is_some() {
                button {
                    r#type: "button",
                    class: style::tag_remove,
                    onclick: move |_| {
                        if let Some(handler) = &on_remove {
                            handler.call(());
                        }
                    },
                    IconX {}
                }
            }
        }
    }
}

/// Tag input component for entering multiple values
#[component]
pub fn TagInput(
    /// Current list of tags
    tags: Signal<Vec<String>>,
    /// Called when tags change (add or remove)
    on_change: EventHandler<Vec<String>>,
    /// Size variant
    #[props(default = TagInputSize::Medium)]
    size: TagInputSize,
    /// Optional label
    #[props(default = String::new())]
    label: String,
    /// Placeholder text for the input
    #[props(default = "Add tag...".to_string())]
    placeholder: String,
    /// Error message
    #[props(default = String::new())]
    error: String,
    /// Whether the input is disabled
    #[props(default = false)]
    disabled: bool,
    /// Maximum number of tags allowed (0 = unlimited)
    #[props(default = 0)]
    max_tags: usize,
) -> Element {
    #[allow(unused_mut, reason = "only used in wasm")]
    let mut input_value = use_signal(String::new);
    #[allow(unused_mut, reason = "only used in wasm")]
    let mut is_focused = use_signal(|| false);

    let size_class = match size {
        TagInputSize::Small => style::small,
        TagInputSize::Medium => "",
        TagInputSize::Large => style::large,
    };

    let error_class = if error.is_empty() {
        ""
    } else {
        style::has_error
    };

    let can_add_more = move || max_tags == 0 || tags().len() < max_tags;

    let mut add_tag = move |value: String| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() || !can_add_more() {
            input_value.set(String::new());
            return;
        }

        let mut current = tags();
        if !current.contains(&trimmed) {
            current.push(trimmed);
            on_change.call(current);
        }
        input_value.set(String::new());
    };

    let remove_tag = move |index: usize| {
        let mut current = tags();
        if index < current.len() {
            current.remove(index);
            on_change.call(current);
        }
    };

    rsx! {
        div { class: stylance::classes!(style::wrapper, size_class, error_class),
            if !label.is_empty() {
                label { class: style::label, {label.clone()} }
            }

            div {
                class: if is_focused() { stylance::classes!(style::container, style::focused) } else { stylance::classes!(style::container) },
                div { class: style::tags,
                    for (index, tag) in (tags)().into_iter().enumerate() {
                        Tag {
                            key: "{index}",
                            label: tag,
                            on_remove: EventHandler::new(move |_| remove_tag(index)),
                        }
                    }
                }

                input {
                    r#type: "text",
                    class: style::input,
                    placeholder: if can_add_more() { placeholder.clone() } else { String::new() },
                    value: input_value(),
                    disabled: disabled || !can_add_more(),
                    onfocus: move |_| is_focused.set(true),
                    onblur: move |_| {
                        is_focused.set(false);
                        let value = input_value();
                        if !value.is_empty() {
                            add_tag(value);
                        }
                    },
                    oninput: move |evt| {
                        input_value.set(evt.value());
                    },
                    onkeydown: move |evt| {
                        use keyboard_types::Key;
                        match evt.key() {
                            Key::Enter => {
                                evt.prevent_default();
                                add_tag(input_value());
                            }
                            Key::Character(ref c) if c == "," => {
                                evt.prevent_default();
                                add_tag(input_value());
                            }
                            Key::Backspace if input_value().is_empty() => {
                                let current = tags();
                                if !current.is_empty() {
                                    remove_tag(current.len() - 1);
                                }
                            }
                            _ => {}
                        }
                    },
                }
            }

            if !error.is_empty() {
                span { class: style::error_text, {error.clone()} }
            }
        }
    }
}
