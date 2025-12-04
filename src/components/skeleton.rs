//! Skeleton loader component for loading states

use dioxus::prelude::*;

stylance::import_style!(style, "skeleton.module.scss");

/// Shape variants for skeleton loaders
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonVariant {
    /// Rectangular shape (default)
    #[default]
    Rectangle,
    /// Circular shape (for avatars)
    Circle,
    /// Rounded rectangle (for text)
    Text,
}

/// Skeleton - placeholder loading animation
#[component]
pub fn Skeleton(
    /// Visual variant
    #[props(default)]
    variant: SkeletonVariant,
    /// Width (CSS value, e.g., "100px", "100%")
    #[props(into)]
    width: Option<String>,
    /// Height (CSS value, e.g., "20px", "1rem")
    #[props(into)]
    height: Option<String>,
    /// Border radius (CSS value, overrides variant default)
    #[props(into)]
    radius: Option<String>,
) -> Element {
    let variant_class = match variant {
        SkeletonVariant::Rectangle => style::rectangle,
        SkeletonVariant::Circle => style::circle,
        SkeletonVariant::Text => style::text,
    };

    let custom_style = {
        let mut styles = Vec::new();
        if let Some(w) = &width {
            styles.push(format!("width: {w}"));
        }
        if let Some(h) = &height {
            styles.push(format!("height: {h}"));
        }
        if let Some(r) = &radius {
            styles.push(format!("border-radius: {r}"));
        }
        if styles.is_empty() {
            String::new()
        } else {
            styles.join("; ")
        }
    };

    rsx! {
        div {
            class: stylance::classes!(style::skeleton, variant_class),
            style: "{custom_style}",
            aria_hidden: "true",
        }
    }
}

/// SkeletonText - multiple lines of skeleton text
#[component]
pub fn SkeletonText(
    /// Number of lines
    #[props(default = 3)]
    lines: usize,
    /// Gap between lines
    #[props(into)]
    gap: Option<String>,
) -> Element {
    let gap_style = gap.map(|g| format!("gap: {g}")).unwrap_or_default();

    rsx! {
        div { class: style::skeleton_text, style: "{gap_style}",
            for i in 0..lines {
                {
                    let is_last = i == lines - 1 && lines > 1;
                    if is_last {
                        rsx! { Skeleton { variant: SkeletonVariant::Text, width: "60%" } }
                    } else {
                        rsx! { Skeleton { variant: SkeletonVariant::Text } }
                    }
                }
            }
        }
    }
}

/// SkeletonAvatar - circular skeleton for avatar placeholders
#[component]
pub fn SkeletonAvatar(
    /// Size (CSS value, e.g., "40px", "3rem")
    #[props(default = "40px".to_string())]
    size: String,
) -> Element {
    rsx! { Skeleton { variant: SkeletonVariant::Circle, width: size.clone(), height: size } }
}

/// SkeletonCard - card-shaped skeleton with optional header and lines
#[component]
pub fn SkeletonCard(
    /// Show avatar in header
    #[props(default = false)]
    show_avatar: bool,
    /// Number of text lines in body
    #[props(default = 3)]
    lines: usize,
) -> Element {
    rsx! {
        div { class: style::skeleton_card,
            if show_avatar {
                div { class: style::skeleton_card_header,
                    SkeletonAvatar { size: "48px" }
                    div { class: style::skeleton_card_header_text,
                        Skeleton { variant: SkeletonVariant::Text, width: "120px" }
                        Skeleton { variant: SkeletonVariant::Text, width: "80px", height: "0.75rem" }
                    }
                }
            }
            SkeletonText { lines }
        }
    }
}

/// SkeletonMessage - skeleton for a chat message row
#[component]
pub fn SkeletonMessage(
    /// Whether to show on the right side (outgoing message)
    #[props(default = false)]
    outgoing: bool,
    /// Number of text lines
    #[props(default = 2)]
    lines: usize,
) -> Element {
    let align_class = if outgoing {
        style::skeleton_message_outgoing
    } else {
        ""
    };

    rsx! {
        div { class: stylance::classes!(style::skeleton_message, align_class),
            if !outgoing {
                SkeletonAvatar { size: "32px" }
            }
            div { class: style::skeleton_message_content,
                Skeleton { variant: SkeletonVariant::Text, width: "80px", height: "0.75rem" }
                div { class: style::skeleton_message_bubble,
                    SkeletonText { lines }
                }
            }
        }
    }
}

/// SkeletonMessageList - skeleton for a list of messages
#[component]
pub fn SkeletonMessageList(
    /// Number of messages to show
    #[props(default = 5)]
    count: usize,
) -> Element {
    rsx! {
        div { class: style::skeleton_message_list,
            for i in 0..count {
                {
                    let outgoing = i % 3 == 2;
                    let lines = if i % 2 == 0 { 2 } else { 1 };
                    rsx! { SkeletonMessage { outgoing, lines } }
                }
            }
        }
    }
}

/// SkeletonThreadItem - skeleton for a thread list item
#[component]
pub fn SkeletonThreadItem() -> Element {
    rsx! {
        div { class: style::skeleton_thread_item,
            SkeletonAvatar { size: "40px" }
            div { class: style::skeleton_thread_item_content,
                div { class: style::skeleton_thread_item_header,
                    Skeleton { variant: SkeletonVariant::Text, width: "120px" }
                    Skeleton { variant: SkeletonVariant::Text, width: "50px", height: "0.75rem" }
                }
                Skeleton { variant: SkeletonVariant::Text, width: "90%", height: "0.75rem" }
            }
        }
    }
}

/// SkeletonThreadList - skeleton for a list of threads
#[component]
pub fn SkeletonThreadList(
    /// Number of threads to show
    #[props(default = 6)]
    count: usize,
) -> Element {
    rsx! {
        div { class: style::skeleton_thread_list,
            for _ in 0..count {
                SkeletonThreadItem {}
            }
        }
    }
}

/// SkeletonCodeBlock - skeleton for a code block
#[component]
pub fn SkeletonCodeBlock(
    /// Number of code lines
    #[props(default = 8)]
    lines: usize,
    /// Show header with language/filename
    #[props(default = true)]
    show_header: bool,
) -> Element {
    rsx! {
        div { class: style::skeleton_code_block,
            if show_header {
                div { class: style::skeleton_code_header,
                    Skeleton { variant: SkeletonVariant::Text, width: "80px", height: "0.875rem" }
                }
            }
            div { class: style::skeleton_code_content,
                for i in 0..lines {
                    {
                        let width = match i % 5 {
                            0 => "70%",
                            1 => "85%",
                            2 => "40%",
                            3 => "95%",
                            _ => "60%",
                        };
                        rsx! { Skeleton { variant: SkeletonVariant::Text, width, height: "1rem" } }
                    }
                }
            }
        }
    }
}
