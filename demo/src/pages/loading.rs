//! Loading page - Progress, Spinner, CircularProgress

use dioxus::prelude::*;
use glade_dioxus::{
    CircularProgress, Progress, ProgressSize, ProgressVariant, Row, RowAlign, Section, Spinner,
    SpinnerSize, Stack, SubSection,
};

#[component]
pub fn LoadingPage() -> Element {
    rsx! {
        Section { id: "progress".to_string(), title: "Progress".to_string(),
            SubSection { title: "Basic".to_string(),
                Stack {
                    Progress { value: 25.0 }
                    Progress { value: 50.0 }
                    Progress { value: 75.0 }
                    Progress { value: 100.0 }
                }
            }
            SubSection { title: "With Label".to_string(),
                Stack {
                    Progress { value: 33.0, show_label: true }
                    Progress { value: 66.0, show_label: true }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Stack {
                    Progress { value: 60.0, size: ProgressSize::Small }
                    Progress { value: 60.0, size: ProgressSize::Medium }
                    Progress { value: 60.0, size: ProgressSize::Large }
                }
            }
            SubSection { title: "Variants".to_string(),
                Stack {
                    Progress { value: 60.0, variant: ProgressVariant::Default }
                    Progress { value: 60.0, variant: ProgressVariant::Success }
                    Progress { value: 60.0, variant: ProgressVariant::Warning }
                    Progress { value: 60.0, variant: ProgressVariant::Error }
                }
            }
            SubSection { title: "Indeterminate".to_string(),
                Progress { value: 0.0, indeterminate: true }
            }
            SubSection { title: "Circular".to_string(),
                Row { align: RowAlign::Center,
                    CircularProgress { value: 25.0 }
                    CircularProgress { value: 50.0, show_label: true }
                    CircularProgress { value: 75.0, variant: ProgressVariant::Success }
                    CircularProgress { value: 0.0, indeterminate: true }
                }
            }
        }

        Section { id: "spinner".to_string(), title: "Spinner".to_string(),
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Stack {
                        span { "Small" }
                        Spinner { size: SpinnerSize::Small }
                    }
                    Stack {
                        span { "Medium" }
                        Spinner { size: SpinnerSize::Medium }
                    }
                    Stack {
                        span { "Large" }
                        Spinner { size: SpinnerSize::Large }
                    }
                }
            }
        }
    }
}
