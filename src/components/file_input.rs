//! File input component with drag-and-drop support

use dioxus::prelude::*;

stylance::import_style!(style, "file_input.module.scss");

/// Size variants for file input
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum FileInputSize {
    /// Small file input zone
    Small,
    /// Medium file input zone (default)
    #[default]
    Medium,
    /// Large file input zone
    Large,
}

/// File input with drag-and-drop zone
#[component]
pub fn FileInput(
    /// Accepted file types (e.g., "image/*", ".pdf,.doc")
    #[props(optional, into)]
    accept: Option<String>,
    /// Allow multiple file selection
    #[props(default = false)]
    multiple: bool,
    /// Callback when files are selected
    on_change: EventHandler<Vec<String>>,
    /// Size variant
    #[props(default)]
    size: FileInputSize,
    /// Whether the input is disabled
    #[props(default = false)]
    disabled: bool,
    /// Custom placeholder text
    #[props(optional, into)]
    placeholder: Option<String>,
    /// Input name attribute
    #[props(optional, into)]
    name: Option<String>,
    /// Input id attribute
    #[props(optional, into)]
    id: Option<String>,
) -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut selected_files = use_signal(Vec::<String>::new);

    let size_class = match size {
        FileInputSize::Small => style::small,
        FileInputSize::Medium => "",
        FileInputSize::Large => style::large,
    };

    let placeholder_text = placeholder.unwrap_or_else(|| {
        if multiple {
            "Drop files here or click to browse".to_string()
        } else {
            "Drop a file here or click to browse".to_string()
        }
    });

    let mut handle_files = move |file_names: Vec<String>| {
        selected_files.set(file_names.clone());
        on_change.call(file_names);
    };

    rsx! {
        div {
            class: stylance::classes!(
                style::file_input,
                size_class,
                if is_dragging() { style::dragging } else { "" },
                if disabled { style::disabled } else { "" }
            ),

            // Hidden actual file input
            input {
                r#type: "file",
                class: style::hidden_input,
                accept: accept.clone(),
                multiple,
                disabled,
                name,
                id: id.clone(),
                onchange: move |evt| {
                    let files = evt.files();
                    if !files.is_empty() {
                        let file_names: Vec<String> = files.iter().map(|f| f.name().to_string()).collect();
                        handle_files(file_names);
                    }
                },
            }

            // Drop zone
            label {
                class: style::drop_zone,
                r#for: id,
                ondragover: move |evt| {
                    evt.prevent_default();
                    if !disabled {
                        is_dragging.set(true);
                    }
                },
                ondragleave: move |_| {
                    is_dragging.set(false);
                },
                ondrop: move |evt| {
                    evt.prevent_default();
                    is_dragging.set(false);
                    // Note: drag-drop file access requires additional setup in Dioxus
                },

                // Upload icon
                div { class: style::icon,
                    svg {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "17 8 12 3 7 8" }
                        line { x1: "12", y1: "3", x2: "12", y2: "15" }
                    }
                }

                // Text content
                div { class: style::content,
                    if selected_files().is_empty() {
                        span { class: style::placeholder, "{placeholder_text}" }
                        if let Some(acc) = &accept {
                            span { class: style::hint, "Accepts: {acc}" }
                        }
                    } else {
                        for file in selected_files() {
                            span { class: style::file_name, "{file}" }
                        }
                    }
                }
            }
        }
    }
}

/// Compact file input button
#[component]
pub fn FileInputButton(
    /// Button label
    #[props(into, default = "Choose file".to_string())]
    label: String,
    /// Accepted file types
    #[props(optional, into)]
    accept: Option<String>,
    /// Allow multiple files
    #[props(default = false)]
    multiple: bool,
    /// Callback when files are selected
    on_change: EventHandler<Vec<String>>,
    /// Whether disabled
    #[props(default = false)]
    disabled: bool,
    /// Input name
    #[props(optional, into)]
    name: Option<String>,
) -> Element {
    let mut selected_files = use_signal(Vec::<String>::new);
    // Simple unique ID generation without rand dependency
    static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let id_num = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let input_id = use_signal(move || format!("file-input-{}", id_num));

    rsx! {
        div { class: style::file_input_button,
            input {
                r#type: "file",
                class: style::hidden_input,
                id: input_id(),
                accept,
                multiple,
                disabled,
                name,
                onchange: move |evt| {
                    let files = evt.files();
                    if !files.is_empty() {
                        let file_names: Vec<String> = files.iter().map(|f| f.name().to_string()).collect();
                        selected_files.set(file_names.clone());
                        on_change.call(file_names);
                    }
                },
            }

            label {
                class: stylance::classes!(style::button, if disabled { style::disabled } else { "" }),
                r#for: input_id(),
                // File icon
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                    polyline { points: "14 2 14 8 20 8" }
                }
                "{label}"
            }

            if !selected_files().is_empty() {
                span { class: style::selected_label,
                    if selected_files().len() == 1 {
                        "{selected_files()[0]}"
                    } else {
                        "{selected_files().len()} files selected"
                    }
                }
            }
        }
    }
}
