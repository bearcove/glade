//! Feedback page - Alert, Toast, Tooltip, EmptyState

use dioxus::prelude::*;
use glade_dioxus::{
    Alert, AlertVariant, Button, ButtonVariant, EmptyState, Row, Section, Stack, SubSection, Toast,
    ToastVariant, Tooltip, TooltipPosition,
};

#[component]
pub fn FeedbackPage() -> Element {
    rsx! {
        Section { id: "alert".to_string(), title: "Alert".to_string(),
            Stack {
                Alert { variant: AlertVariant::Info, title: "Info".to_string(), "This is an informational message." }
                Alert { variant: AlertVariant::Success, title: "Success".to_string(), "Operation completed successfully!" }
                Alert { variant: AlertVariant::Warning, title: "Warning".to_string(), "Please review before continuing." }
                Alert { variant: AlertVariant::Error, title: "Error".to_string(), "Something went wrong." }
                Alert { variant: AlertVariant::Info, dismissible: true, "Dismissible alert" }
            }
        }

        Section { id: "tooltip".to_string(), title: "Tooltip".to_string(),
            SubSection { title: "Positions".to_string(),
                Row {
                    Tooltip { text: "Tooltip on top".to_string(), position: TooltipPosition::Top,
                        Button { variant: ButtonVariant::Secondary, "Top" }
                    }
                    Tooltip { text: "Tooltip on bottom".to_string(), position: TooltipPosition::Bottom,
                        Button { variant: ButtonVariant::Secondary, "Bottom" }
                    }
                    Tooltip { text: "Tooltip on left".to_string(), position: TooltipPosition::Left,
                        Button { variant: ButtonVariant::Secondary, "Left" }
                    }
                    Tooltip { text: "Tooltip on right".to_string(), position: TooltipPosition::Right,
                        Button { variant: ButtonVariant::Secondary, "Right" }
                    }
                }
            }
        }

        Section { id: "empty-state".to_string(), title: "Empty State".to_string(),
            EmptyState {
                title: "No results found".to_string(),
                description: "Try adjusting your search or filters.".to_string(),
                action: rsx! { Button { "Clear filters" } },
            }
        }

        Section { id: "toast".to_string(), title: "Toast".to_string(),
            SubSection { title: "Variants".to_string(),
                Stack {
                    Toast { variant: ToastVariant::Info, title: Some("Info".to_string()),
                        "This is an informational message."
                    }
                    Toast { variant: ToastVariant::Success, title: Some("Success".to_string()),
                        "Operation completed successfully!"
                    }
                    Toast { variant: ToastVariant::Warning, title: Some("Warning".to_string()),
                        "Please review before continuing."
                    }
                    Toast { variant: ToastVariant::Error, title: Some("Error".to_string()),
                        "Something went wrong."
                    }
                }
            }
        }
    }
}
