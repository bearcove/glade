//! Code execution result panel component

use dioxus::prelude::*;

use crate::{IconCheck, IconX, Spinner, SpinnerSize, Tab, TabList, TabPanel, TabPanels, Tabs};

stylance::import_style!(style, "code_execution_result.module.scss");

/// Status of code execution
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionStatus {
    /// Currently running
    #[default]
    Running,
    /// Completed successfully
    Success,
    /// Failed with error
    Failure,
}

/// Code execution result panel
#[component]
pub fn CodeExecutionResult(
    /// Execution status
    #[props(default)]
    status: ExecutionStatus,
    /// Exit code (if finished)
    exit_code: Option<i32>,
    /// Duration string (e.g., "1.2s")
    duration: Option<String>,
    /// Stdout content
    #[props(default)]
    stdout: String,
    /// Stderr content
    #[props(default)]
    stderr: String,
) -> Element {
    let status_class = match status {
        ExecutionStatus::Running => style::running,
        ExecutionStatus::Success => style::success,
        ExecutionStatus::Failure => style::failure,
    };

    let status_text = match status {
        ExecutionStatus::Running => "Running",
        ExecutionStatus::Success => "Success",
        ExecutionStatus::Failure => "Failed",
    };

    let has_stderr = !stderr.is_empty();

    rsx! {
        div { class: stylance::classes!(style::panel, status_class),
            div { class: style::header,
                div { class: style::status,
                    span { class: style::status_icon,
                        match status {
                            ExecutionStatus::Running => rsx! { Spinner { size: SpinnerSize::Small } },
                            ExecutionStatus::Success => rsx! { IconCheck {} },
                            ExecutionStatus::Failure => rsx! { IconX {} },
                        }
                    }
                    span { class: style::status_text, "{status_text}" }
                }
                div { class: style::meta,
                    if let Some(d) = &duration {
                        span { class: style::duration, "{d}" }
                    }
                    if let Some(code) = exit_code {
                        span { class: style::exit_code, "Exit code: {code}" }
                    }
                }
            }
            div { class: style::content,
                Tabs {
                    TabList {
                        Tab { "Output" }
                        if has_stderr {
                            Tab { "Errors" }
                        }
                    }
                    TabPanels {
                        TabPanel {
                            pre { class: style::output, "{stdout}" }
                        }
                        if has_stderr {
                            TabPanel {
                                pre { class: stylance::classes!(style::output, style::stderr), "{stderr}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
