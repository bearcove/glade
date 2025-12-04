//! `RetryButton` component for retrying failed actions

use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, IconLoader};

stylance::import_style!(style, "retry_button.module.scss");

/// State for the retry button
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum RetryButtonState {
    /// Ready to perform action
    #[default]
    Idle,
    /// Action in progress
    Loading,
    /// Action failed, can retry
    Failed,
    /// Action succeeded
    Success,
}

/// A button that handles retry logic for failed actions
#[component]
pub fn RetryButton(
    /// Current state of the button
    state: RetryButtonState,
    /// Called when the button is clicked (initial or retry)
    on_click: EventHandler<()>,
    /// Button size
    #[props(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Text to show in idle state
    #[props(default = "Submit".to_string())]
    idle_text: String,
    /// Text to show while loading
    #[props(default = "Loading...".to_string())]
    loading_text: String,
    /// Text to show when failed (retry prompt)
    #[props(default = "Retry".to_string())]
    retry_text: String,
    /// Text to show on success
    #[props(default = "Done".to_string())]
    success_text: String,
) -> Element {
    let is_loading = state == RetryButtonState::Loading;
    let is_disabled = state == RetryButtonState::Loading || state == RetryButtonState::Success;

    let variant = match state {
        RetryButtonState::Idle | RetryButtonState::Loading | RetryButtonState::Success => {
            ButtonVariant::Primary
        }
        RetryButtonState::Failed => ButtonVariant::Danger,
    };

    let text = match state {
        RetryButtonState::Idle => idle_text,
        RetryButtonState::Loading => loading_text,
        RetryButtonState::Failed => retry_text,
        RetryButtonState::Success => success_text,
    };

    rsx! {
        Button {
            variant,
            size,
            disabled: is_disabled,
            onclick: move |_| on_click.call(()),
            span { class: style::content,
                if is_loading {
                    span { class: style::spinner, IconLoader {} }
                }
                span { "{text}" }
            }
        }
    }
}
