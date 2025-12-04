//! Copy input component - displays text with a clipboard button

use dioxus::prelude::*;

stylance::import_style!(style, "copy_input.module.scss");

/// Size variants for the CopyInput
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyInputSize {
    /// Small size
    Small,
    /// Medium size (default)
    #[default]
    Medium,
}

/// A read-only input with a copy-to-clipboard button.
/// Useful for displaying registration numbers, codes, URLs, etc.
#[component]
pub fn CopyInput(
    /// The text displayed to the user (can include formatting like spaces)
    display: String,
    /// The text copied to clipboard (defaults to display if not provided)
    #[props(default)]
    value: String,
    /// Optional label displayed above the input
    #[props(default)]
    label: String,
    /// Size variant
    #[props(default)]
    size: CopyInputSize,
) -> Element {
    // Needs `mut` for wasm32 (where .set() is called) but not server
    #[allow(unused_mut)]
    let mut copied = use_signal(|| false);

    // Use value if provided, otherwise use display
    #[cfg(target_arch = "wasm32")]
    let copy_value = if value.is_empty() { &display } else { &value };

    let size_class = match size {
        CopyInputSize::Small => style::small,
        CopyInputSize::Medium => "",
    };

    rsx! {
        div { class: stylance::classes!(style::wrapper, size_class),
            if !label.is_empty() {
                label { class: style::label, "{label}" }
            }
            div { class: style::container,
                span { class: style::text, "{display}" }
                button {
                    r#type: "button",
                    class: stylance::classes!(style::copy_button, if copied() { style::copied } else { "" }),
                    onclick: {
                        #[cfg(target_arch = "wasm32")]
                        let copy_value = copy_value.to_string();
                        move |_| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(window) = web_sys::window() {
                                    let clipboard = window.navigator().clipboard();
                                    let _ = clipboard.write_text(&copy_value);
                                    copied.set(true);

                                    spawn(async move {
                                        gloo_timers::future::TimeoutFuture::new(2_000).await;
                                        copied.set(false);
                                    });
                                }
                            }
                        }
                    },
                    if copied() {
                        // Check icon
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                    } else {
                        // Copy icon
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect { x: "9", y: "9", width: "13", height: "13", rx: "2", ry: "2" }
                            path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                        }
                    }
                }
            }
        }
    }
}
