//! Tool call badge component for displaying tool execution status

use dioxus::prelude::*;

use crate::{IconCheck, IconLoader, IconX};

stylance::import_style!(style, "tool_call_badge.module.scss");

/// Status of a tool call
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolCallStatus {
    /// Pending execution
    #[default]
    Pending,
    /// Currently running
    Running,
    /// Completed successfully
    Success,
    /// Failed with error
    Error,
    /// Cancelled
    Cancelled,
}

/// Tool call badge showing tool name and status
#[component]
pub fn ToolCallBadge(
    /// Name of the tool
    name: String,
    /// Current status
    #[props(default)]
    status: ToolCallStatus,
    /// Compact mode (icon only)
    #[props(default = false)]
    compact: bool,
) -> Element {
    let status_class = match status {
        ToolCallStatus::Pending => style::pending,
        ToolCallStatus::Running => style::running,
        ToolCallStatus::Success => style::success,
        ToolCallStatus::Error => style::error,
        ToolCallStatus::Cancelled => style::cancelled,
    };

    rsx! {
        span { class: stylance::classes!(style::badge, status_class, if compact { style::compact } else { "" }),
            span { class: style::icon,
                match status {
                    ToolCallStatus::Pending => rsx! { span { class: style::icon_pending } },
                    ToolCallStatus::Running => rsx! { IconLoader {} },
                    ToolCallStatus::Success => rsx! { IconCheck {} },
                    ToolCallStatus::Error | ToolCallStatus::Cancelled => rsx! { IconX {} },
                }
            }
            if !compact {
                span { class: style::name, "{name}" }
            }
        }
    }
}
