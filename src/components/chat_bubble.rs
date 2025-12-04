//! Chat bubble component for message display

use dioxus::prelude::*;

use crate::Avatar;

stylance::import_style!(style, "chat_bubble.module.scss");

/// Alignment for the chat bubble
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ChatBubbleAlign {
    /// Message from others (left-aligned with avatar)
    #[default]
    Start,
    /// Own message (right-aligned, no avatar)
    End,
}

/// Chat bubble variant
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ChatBubbleVariant {
    /// Default bubble style
    #[default]
    Default,
    /// Primary/accent colored bubble (for own messages)
    Primary,
    /// Muted/subtle bubble
    Muted,
}

/// A chat message bubble
#[component]
pub fn ChatBubble(
    /// Message alignment
    #[props(default)]
    align: ChatBubbleAlign,
    /// Visual variant
    #[props(default)]
    variant: ChatBubbleVariant,
    /// Sender name (shown above message for Start alignment)
    sender: Option<String>,
    /// Avatar image URL
    avatar: Option<String>,
    /// Avatar initials (fallback if no image)
    initials: Option<String>,
    /// Timestamp to display
    timestamp: Option<String>,
    /// Whether this message is part of a group (hides avatar/sender)
    #[props(default = false)]
    grouped: bool,
    /// Message content
    children: Element,
) -> Element {
    let align_class = match align {
        ChatBubbleAlign::Start => style::start,
        ChatBubbleAlign::End => style::end,
    };

    let variant_class = match variant {
        ChatBubbleVariant::Default => style::variant_default,
        ChatBubbleVariant::Primary => style::primary,
        ChatBubbleVariant::Muted => style::muted,
    };

    let show_avatar = align == ChatBubbleAlign::Start && !grouped;
    let show_sender = align == ChatBubbleAlign::Start && sender.is_some() && !grouped;

    rsx! {
        div { class: stylance::classes!(style::chat_bubble, align_class),
            if show_avatar {
                div { class: style::avatar,
                    if let Some(url) = &avatar {
                        Avatar { src: url.clone() }
                    } else {
                        Avatar {
                            initials: initials.clone().unwrap_or_else(|| "?".to_string()),
                        }
                    }
                }
            }
            if !show_avatar && align == ChatBubbleAlign::Start && grouped {
                div { class: style::avatar_spacer }
            }
            div { class: style::content,
                if show_sender {
                    if let Some(name) = &sender {
                        span { class: style::sender, "{name}" }
                    }
                }
                div { class: stylance::classes!(style::bubble, variant_class), {children} }
                if let Some(ts) = timestamp {
                    span { class: style::timestamp, "{ts}" }
                }
            }
        }
    }
}

/// A row containing a message with optional reactions and actions
#[component]
pub fn MessageRow(
    /// Whether to show on hover actions
    #[props(default = false)]
    show_actions: bool,
    /// Actions slot (buttons, etc.)
    actions: Option<Element>,
    /// The message content
    children: Element,
) -> Element {
    rsx! {
        div { class: style::message_row,
            div { class: style::message_content, {children} }
            if show_actions && actions.is_some() {
                div { class: style::message_actions, {actions} }
            }
        }
    }
}
