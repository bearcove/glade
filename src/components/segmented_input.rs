//! Segmented input component for TOTP/OTP codes

use dioxus::prelude::*;
use std::rc::Rc;

stylance::import_style!(style, "segmented_input.module.scss");

/// Size variants for the SegmentedInput
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SegmentedInputSize {
    /// Small segments
    Small,
    /// Medium segments (default)
    #[default]
    Medium,
    /// Large segments
    Large,
}

/// Segmented input for TOTP/OTP codes
#[component]
pub fn SegmentedInput(
    /// Number of segments (default: 6)
    #[props(default = 6)]
    length: usize,
    /// Current value
    #[props(default = String::new())]
    value: String,
    /// Called when value changes
    #[props(optional)]
    on_change: Option<EventHandler<String>>,
    /// Called when all segments are filled
    #[props(optional)]
    on_complete: Option<EventHandler<String>>,
    /// Size variant
    #[props(default = SegmentedInputSize::Medium)]
    size: SegmentedInputSize,
    /// Whether the input is disabled
    #[props(default = false)]
    disabled: bool,
    /// Show separator after this many digits (0 = no separator)
    #[props(default = 3)]
    separator_after: usize,
    /// Label text
    #[props(default = String::new())]
    label: String,
    /// Error message
    #[props(default = String::new())]
    error: String,
) -> Element {
    // segments state
    let mut segments = use_signal(|| {
        value
            .chars()
            .take(length)
            .map(|c| c.to_string())
            .chain(std::iter::repeat(String::new()))
            .take(length)
            .collect::<Vec<_>>()
    });

    // Store MountedData for each segment
    let mut mounted_inputs: Signal<Vec<Option<Rc<MountedData>>>> =
        use_signal(|| vec![None; length]);

    let size_class = match size {
        SegmentedInputSize::Small => style::small,
        SegmentedInputSize::Medium => "",
        SegmentedInputSize::Large => style::large,
    };

    let error_class = if error.is_empty() {
        ""
    } else {
        style::has_error
    };

    let focus_segment = move |index: usize| {
        if let Some(Some(mounted)) = mounted_inputs.read().get(index).cloned() {
            spawn(async move {
                let _ = mounted.set_focus(true).await;
            });
        }
    };

    let notify_change = move |new_segments: Vec<String>| {
        let full_value = new_segments.join("");
        if let Some(cb) = &on_change {
            cb.call(full_value.clone());
        }
        if new_segments.iter().all(|s| !s.is_empty()) {
            if let Some(cb) = &on_complete {
                cb.call(full_value);
            }
        }
    };

    let mut handle_paste = move |index: usize, pasted: String| {
        let mut current = segments();
        let chars: Vec<char> = pasted
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .take(length)
            .collect();

        let start_index = if chars.len() >= length { 0 } else { index };

        for (i, ch) in chars.iter().enumerate() {
            let target_idx = start_index + i;
            if target_idx < length {
                current[target_idx] = ch.to_string().to_uppercase();
            }
        }

        segments.set(current.clone());
        notify_change(current);

        let filled_count = start_index + chars.len();
        let focus_idx = filled_count.min(length.saturating_sub(1));
        focus_segment(focus_idx);
    };

    let mut handle_input = move |index: usize, input_value: String| {
        if input_value.len() > 1 {
            handle_paste(index, input_value);
            return;
        }

        let mut current = segments();
        if let Some(ch) = input_value.chars().next() {
            if ch.is_ascii_alphanumeric() {
                current[index] = ch.to_string().to_uppercase();
                segments.set(current.clone());
                notify_change(current);

                if index < length - 1 {
                    focus_segment(index + 1);
                }
            }
        }
    };

    let mut handle_keydown = move |index: usize, evt: KeyboardEvent| {
        use keyboard_types::Key;
        match evt.key() {
            Key::Backspace => {
                let mut current = segments();
                if current[index].is_empty() && index > 0 {
                    current[index - 1] = String::new();
                    segments.set(current.clone());
                    notify_change(current);
                    focus_segment(index - 1);
                } else {
                    current[index] = String::new();
                    segments.set(current.clone());
                    notify_change(current);
                }
            }
            Key::ArrowLeft if index > 0 => focus_segment(index - 1),
            Key::ArrowRight if index + 1 < length => focus_segment(index + 1),
            _ => {}
        }
    };

    rsx! {
        div { class: stylance::classes!(style::wrapper, size_class, error_class),
            if !label.is_empty() {
                label { class: style::label, {label.clone()} }
            }

            div { class: style::segments,
                {(0..length).map(|i| {
                    let show_separator = separator_after > 0 && i > 0 && i % separator_after == 0;
                    rsx! {
                        if show_separator {
                            span { class: style::separator, "-" }
                        }
                        input {
                            r#type: "text",
                            inputmode: "numeric",
                            autocomplete: "one-time-code",
                            class: style::segment,
                            disabled: disabled,
                            value: segments().get(i).cloned().unwrap_or_default(),
                            onmounted: move |evt| {
                                mounted_inputs.write()[i] = Some(evt.data());
                            },
                            onpaste: move |evt| {
                                evt.prevent_default();
                                #[cfg(target_arch = "wasm32")]
                                {
                                    use crate::utils::ClipboardEventExt;
                                    if let Some(text) = evt.text() {
                                        handle_paste(i, text);
                                    }
                                }
                            },
                            oninput: move |evt| handle_input(i, evt.value()),
                            onkeydown: move |evt| handle_keydown(i, evt),
                            onfocus: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                if let Some(Some(mounted)) = mounted_inputs.read().get(i).cloned() {
                                    use crate::utils::MountedInputExt;
                                    mounted.select();
                                }
                            },
                        }
                    }
                })}
            }

            if !error.is_empty() {
                span { class: style::error_text, {error.clone()} }
            }
        }
    }
}
