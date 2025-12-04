//! Spinner component for loading indicators

use dioxus::prelude::*;

stylance::import_style!(style, "spinner.module.scss");

/// Size variants for Spinner
#[derive(Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    /// Small spinner (16px)
    Small,
    /// Medium spinner (24px)
    Medium,
    /// Large spinner (32px)
    Large,
}

#[component]
pub fn Spinner(#[props(default = SpinnerSize::Medium)] size: SpinnerSize) -> Element {
    let size_class = match size {
        SpinnerSize::Small => style::small,
        SpinnerSize::Medium => "",
        SpinnerSize::Large => style::large,
    };

    rsx! {
        div {
            class: stylance::classes!(style::spinner, size_class),
            div {
                class: style::circle,
            }
        }
    }
}
