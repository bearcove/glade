//! Chat page - ChatBubble, MessageComposer, MessageGroup, MessageList, ThreadList, etc.

use dioxus::prelude::*;
use glade::{
    AttachmentChip, AttachmentType, ChatBubble, ChatBubbleAlign, ChatBubbleVariant,
    DayDivider, GroupMessage, MessageComposer, MessageGroup, MessageList, NewMessagesDivider,
    NotificationBadge, NotificationBadgeSize, Row, RowAlign, Section, Stack, StreamingStatus,
    StreamingStatusVariant, SubSection, ThreadList, ThreadListHeader, ThreadListItem,
    ThreadListSection,
};

#[component]
pub fn ChatPage() -> Element {
    let composer_value = use_signal(String::new);

    rsx! {
        Section { id: "chat-bubble".to_string(), title: "Chat Bubble".to_string(),
            SubSection { title: "Alignments".to_string(),
                Stack {
                    ChatBubble { align: ChatBubbleAlign::Start, sender: Some("Alice".to_string()),
                        "Hello! How are you doing today?"
                    }
                    ChatBubble { align: ChatBubbleAlign::End, variant: ChatBubbleVariant::Primary,
                        "I'm doing great, thanks for asking!"
                    }
                    ChatBubble { align: ChatBubbleAlign::Start, sender: Some("Alice".to_string()), timestamp: Some("2:34 PM".to_string()),
                        "That's wonderful to hear."
                    }
                }
            }
            SubSection { title: "Variants".to_string(),
                Stack {
                    ChatBubble { variant: ChatBubbleVariant::Default,
                        "Default variant"
                    }
                    ChatBubble { variant: ChatBubbleVariant::Primary,
                        "Primary variant"
                    }
                    ChatBubble { variant: ChatBubbleVariant::Muted,
                        "Muted variant"
                    }
                }
            }
            SubSection { title: "Grouped Messages".to_string(),
                Stack {
                    ChatBubble {
                        align: ChatBubbleAlign::Start,
                        sender: Some("Bob".to_string()),
                        avatar: Some("https://placecats.com/bella/64/64".to_string()),
                        "First message in a group"
                    }
                    ChatBubble { align: ChatBubbleAlign::Start, grouped: true,
                        "Second message, grouped"
                    }
                    ChatBubble { align: ChatBubbleAlign::Start, grouped: true, timestamp: Some("3:45 PM".to_string()),
                        "Third message with timestamp"
                    }
                }
            }
        }

        Section { id: "message-group".to_string(), title: "Message Group".to_string(),
            MessageGroup {
                author: "Claude".to_string(),
                avatar: Some("https://placecats.com/millie/64/64".to_string()),
                timestamp: Some("Just now".to_string()),
                GroupMessage { "Here's a response to your question." }
                GroupMessage { "I've broken it into multiple paragraphs for clarity." }
            }
            MessageGroup {
                author: "You".to_string(),
                avatar: Some("https://placecats.com/neo/64/64".to_string()),
                is_self: true,
                timestamp: Some("2 min ago".to_string()),
                GroupMessage { "Thanks for the explanation!" }
            }
        }

        Section { id: "message-list".to_string(), title: "Message List".to_string(),
            div { style: "height: 400px; border: 1px solid var(--color-border); border-radius: 0.5rem;",
                MessageList {
                    DayDivider { label: "Today".to_string() }
                    MessageGroup {
                        author: "Alice".to_string(),
                        avatar: Some("https://placecats.com/millie/64/64".to_string()),
                        timestamp: Some("9:00 AM".to_string()),
                        GroupMessage { "Good morning!" }
                    }
                    MessageGroup {
                        author: "You".to_string(),
                        avatar: Some("https://placecats.com/neo/64/64".to_string()),
                        is_self: true,
                        timestamp: Some("9:01 AM".to_string()),
                        GroupMessage { "Good morning! How can I help?" }
                    }
                    NewMessagesDivider {}
                    MessageGroup {
                        author: "Alice".to_string(),
                        avatar: Some("https://placecats.com/millie/64/64".to_string()),
                        timestamp: Some("9:05 AM".to_string()),
                        GroupMessage { "I have a question about the project." }
                        GroupMessage { "Can we schedule a call later today?" }
                    }
                }
            }
        }

        Section { id: "message-composer".to_string(), title: "Message Composer".to_string(),
            SubSection { title: "Basic".to_string(),
                MessageComposer {
                    value: composer_value,
                    on_send: move |_msg: String| {
                        // Message sent
                    },
                }
            }
            SubSection { title: "Disabled / Sending".to_string(),
                Stack {
                    MessageComposer {
                        value: use_signal(|| "Sending...".to_string()),
                        is_sending: true,
                    }
                    MessageComposer {
                        value: use_signal(String::new),
                        disabled: true,
                        placeholder: "Disabled composer".to_string(),
                    }
                }
            }
        }

        Section { id: "thread-list".to_string(), title: "Thread List".to_string(),
            div { style: "max-width: 320px; border: 1px solid var(--color-border); border-radius: 0.5rem;",
                ThreadList {
                    header: rsx! {
                        ThreadListHeader { title: "Conversations".to_string() }
                    },
                    ThreadListSection { label: "Pinned".to_string() }
                    ThreadListItem {
                        title: "Project Discussion".to_string(),
                        snippet: Some("Let's review the timeline...".to_string()),
                        timestamp: Some("2m ago".to_string()),
                        pinned: true,
                    }
                    ThreadListSection { label: "Recent".to_string() }
                    ThreadListItem {
                        title: "Alice".to_string(),
                        snippet: Some("Thanks for your help!".to_string()),
                        timestamp: Some("10m ago".to_string()),
                        unread_count: Some(3),
                    }
                    ThreadListItem {
                        title: "Team Chat".to_string(),
                        snippet: Some("Meeting at 3pm".to_string()),
                        timestamp: Some("1h ago".to_string()),
                        selected: true,
                    }
                    ThreadListItem {
                        title: "Bob".to_string(),
                        snippet: Some("See you tomorrow".to_string()),
                        timestamp: Some("Yesterday".to_string()),
                    }
                }
            }
        }

        Section { id: "day-divider".to_string(), title: "Day Divider".to_string(),
            Stack {
                DayDivider { label: "Today".to_string() }
                DayDivider { label: "Yesterday".to_string() }
                DayDivider { label: "March 15, 2024".to_string() }
                NewMessagesDivider {}
                NewMessagesDivider { label: "3 new messages".to_string() }
            }
        }

        Section { id: "streaming-status".to_string(), title: "Streaming Status".to_string(),
            SubSection { title: "Variants".to_string(),
                Stack {
                    StreamingStatus { variant: StreamingStatusVariant::Thinking }
                    StreamingStatus { variant: StreamingStatusVariant::Searching }
                    StreamingStatus { variant: StreamingStatusVariant::Running }
                    StreamingStatus { variant: StreamingStatusVariant::Writing }
                    StreamingStatus { variant: StreamingStatusVariant::Custom, text: "Processing files".to_string() }
                }
            }
            SubSection { title: "Without Icon".to_string(),
                StreamingStatus { variant: StreamingStatusVariant::Thinking, show_icon: false }
            }
        }

        Section { id: "attachment-chip".to_string(), title: "Attachment Chip".to_string(),
            SubSection { title: "Types".to_string(),
                Row { align: RowAlign::Center,
                    AttachmentChip { name: "document.pdf".to_string(), size: Some("2.5 MB".to_string()) }
                    AttachmentChip { name: "photo.jpg".to_string(), attachment_type: AttachmentType::Image }
                    AttachmentChip { name: "main.rs".to_string(), attachment_type: AttachmentType::Context }
                }
            }
            SubSection { title: "With Remove".to_string(),
                Row { align: RowAlign::Center,
                    AttachmentChip {
                        name: "removable.txt".to_string(),
                        on_remove: move |_| { /* Remove clicked */ },
                    }
                    AttachmentChip {
                        name: "disabled.txt".to_string(),
                        disabled: true,
                        on_remove: move |_| {},
                    }
                }
            }
        }

        Section { id: "notification-badge".to_string(), title: "Notification Badge".to_string(),
            SubSection { title: "Counts".to_string(),
                Row { align: RowAlign::Center,
                    NotificationBadge { count: Some(1) }
                    NotificationBadge { count: Some(5) }
                    NotificationBadge { count: Some(42) }
                    NotificationBadge { count: Some(100) }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    NotificationBadge { count: Some(3), size: NotificationBadgeSize::Small }
                    NotificationBadge { count: Some(3), size: NotificationBadgeSize::Medium }
                    NotificationBadge { count: Some(3), size: NotificationBadgeSize::Large }
                }
            }
            SubSection { title: "Dot Only".to_string(),
                Row { align: RowAlign::Center,
                    NotificationBadge { count: None }
                    NotificationBadge { count: None, size: NotificationBadgeSize::Large }
                }
            }
        }
    }
}
