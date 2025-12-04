//! Thread list components for conversation navigation

use dioxus::prelude::*;

use crate::NotificationBadge;

stylance::import_style!(style, "thread_list.module.scss");

/// Thread list container
#[component]
pub fn ThreadList(
    /// Optional header content
    header: Option<Element>,
    /// Thread items
    children: Element,
) -> Element {
    rsx! {
        div { class: style::thread_list,
            if let Some(h) = header {
                div { class: style::header, {h} }
            }
            div { class: style::items, {children} }
        }
    }
}

/// Header for thread list
#[component]
pub fn ThreadListHeader(
    /// Title text
    #[props(default = "Conversations".to_string())]
    title: String,
    /// Optional action buttons (right side)
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: style::list_header,
            h2 { class: style::list_title, "{title}" }
            if let Some(a) = actions {
                div { class: style::list_actions, {a} }
            }
        }
    }
}

/// Section header within thread list
#[component]
pub fn ThreadListSection(
    /// Section label
    label: String,
) -> Element {
    rsx! {
        div { class: style::section, "{label}" }
    }
}

/// Individual thread item
#[component]
pub fn ThreadListItem(
    /// Thread title
    title: String,
    /// Last message snippet
    snippet: Option<String>,
    /// Timestamp (e.g., "2m ago", "Yesterday")
    timestamp: Option<String>,
    /// Unread message count (None = read)
    unread_count: Option<u32>,
    /// Whether this item is selected
    #[props(default = false)]
    selected: bool,
    /// Whether this item is pinned
    #[props(default = false)]
    pinned: bool,
    /// Click handler
    on_click: Option<EventHandler<()>>,
) -> Element {
    let selected_class = if selected { style::selected } else { "" };
    let pinned_class = if pinned { style::pinned } else { "" };
    let unread_class = if unread_count.is_some() {
        style::unread
    } else {
        ""
    };

    rsx! {
        button {
            r#type: "button",
            class: stylance::classes!(style::item, selected_class, pinned_class, unread_class),
            onclick: move |_| {
                if let Some(cb) = &on_click {
                    cb.call(());
                }
            },
            div { class: style::item_content,
                div { class: style::item_header,
                    span { class: style::item_title, "{title}" }
                    if let Some(t) = &timestamp {
                        span { class: style::item_time, "{t}" }
                    }
                }
                if let Some(s) = &snippet {
                    p { class: style::item_snippet, "{s}" }
                }
            }
            if let Some(count) = unread_count {
                div { class: style::item_badge,
                    NotificationBadge { count: Some(count) }
                }
            }
        }
    }
}
