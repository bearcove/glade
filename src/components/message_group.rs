//! Message group for grouping consecutive messages by author

use dioxus::prelude::*;

use crate::{Avatar, AvatarSize};

stylance::import_style!(style, "message_group.module.scss");

/// Message group for consecutive messages from the same author
#[component]
pub fn MessageGroup(
    /// Author name
    author: String,
    /// Author avatar URL (optional)
    avatar: Option<String>,
    /// Timestamp for the group (shown on first message)
    timestamp: Option<String>,
    /// Whether this group is from the current user (right-aligned)
    #[props(default = false)]
    is_self: bool,
    /// Message bubbles
    children: Element,
) -> Element {
    let align_class = if is_self { style::self_group } else { "" };

    let initials = author
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();

    rsx! {
        div { class: stylance::classes!(style::group, align_class),
            // Avatar always rendered first - CSS row-reverse handles positioning for self
            div { class: style::avatar,
                Avatar {
                    src: avatar.clone().unwrap_or_default(),
                    initials: initials.clone(),
                    size: AvatarSize::Small,
                }
            }
            div { class: style::content,
                div { class: style::header,
                    if !is_self {
                        span { class: style::author, "{author}" }
                    }
                    if let Some(t) = &timestamp {
                        span { class: style::timestamp, "{t}" }
                    }
                }
                div { class: style::messages, {children} }
            }
        }
    }
}

/// Individual message within a group (simplified bubble)
#[component]
pub fn GroupMessage(children: Element) -> Element {
    rsx! {
        div { class: style::message, {children} }
    }
}
