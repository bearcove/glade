//! Message composer component for chat input

use dioxus::prelude::*;

use crate::{Button, ButtonVariant};

stylance::import_style!(style, "message_composer.module.scss");

/// Message composer with textarea and send button
#[component]
pub fn MessageComposer(
    /// Placeholder text
    #[props(default = "Type a message...".to_string())]
    placeholder: String,
    /// Current value
    value: Signal<String>,
    /// Called when send is triggered (Enter or button click)
    on_send: Option<EventHandler<String>>,
    /// Whether sending is in progress
    #[props(default = false)]
    is_sending: bool,
    /// Whether the composer is disabled
    #[props(default = false)]
    disabled: bool,
    /// Optional toolbar content (left side)
    toolbar_left: Option<Element>,
    /// Optional toolbar content (right side)
    toolbar_right: Option<Element>,
) -> Element {
    let mut internal_value = value;

    let mut handle_send = move || {
        let text = internal_value();
        if !text.trim().is_empty() {
            if let Some(cb) = &on_send {
                cb.call(text);
            }
            internal_value.set(String::new());
        }
    };

    let has_toolbar = toolbar_left.is_some() || toolbar_right.is_some();

    rsx! {
        div { class: stylance::classes!(style::composer, if is_sending { style::sending } else { "" }),
            if has_toolbar {
                div { class: style::toolbar,
                    div { class: style::toolbar_left, {toolbar_left} }
                    div { class: style::toolbar_right, {toolbar_right} }
                }
            }
            div { class: style::input_row,
                textarea {
                    class: style::textarea,
                    placeholder: "{placeholder}",
                    disabled: disabled || is_sending,
                    value: "{internal_value}",
                    oninput: move |evt| {
                        internal_value.set(evt.value());
                    },
                    onkeydown: move |evt| {
                        // Ctrl+Enter or Cmd+Enter to send
                        if evt.key() == Key::Enter && (evt.modifiers().ctrl() || evt.modifiers().meta()) {
                            evt.prevent_default();
                            handle_send();
                        }
                    },
                }
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: disabled || is_sending || internal_value().trim().is_empty(),
                    loading: is_sending,
                    onclick: move |_| handle_send(),
                    "Send"
                }
            }
        }
    }
}

/// Toolbar button for the composer
#[component]
pub fn ComposerButton(
    /// Button click handler
    on_click: Option<EventHandler<()>>,
    /// Button content
    children: Element,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: style::toolbar_button,
            onclick: move |_| {
                if let Some(cb) = &on_click {
                    cb.call(());
                }
            },
            {children}
        }
    }
}
