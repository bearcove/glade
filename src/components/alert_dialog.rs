//! AlertDialog component for confirmation dialogs

use dioxus::prelude::*;

stylance::import_style!(style, "alert_dialog.module.scss");

/// Variant/intent of the alert dialog
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertDialogVariant {
    /// Default informational dialog
    #[default]
    Default,
    /// Danger/destructive action dialog (red)
    Danger,
    /// Warning dialog (yellow/orange)
    Warning,
}

/// An alert dialog for confirmations and destructive actions
#[component]
pub fn AlertDialog(
    /// Whether the dialog is open
    open: bool,
    /// Close handler
    onclose: EventHandler<()>,
    /// Title of the dialog
    #[props(into)]
    title: String,
    /// Description/message
    #[props(into)]
    description: String,
    /// Cancel button text
    #[props(into, default = "Cancel".to_string())]
    cancel_text: String,
    /// Confirm button text
    #[props(into, default = "Confirm".to_string())]
    confirm_text: String,
    /// Cancel handler (defaults to close)
    #[props(optional)]
    oncancel: Option<EventHandler<()>>,
    /// Confirm handler
    onconfirm: EventHandler<()>,
    /// Variant/intent
    #[props(default)]
    variant: AlertDialogVariant,
    /// Whether confirm is loading
    #[props(default = false)]
    loading: bool,
    /// Optional icon
    #[props(optional)]
    icon: Option<Element>,
) -> Element {
    if !open {
        return rsx! {};
    }

    let variant_class = match variant {
        AlertDialogVariant::Default => "",
        AlertDialogVariant::Danger => style::danger,
        AlertDialogVariant::Warning => style::warning,
    };

    let handle_cancel = move |_| {
        if let Some(handler) = &oncancel {
            handler.call(());
        } else {
            onclose.call(());
        }
    };

    let handle_confirm = move |_| {
        onconfirm.call(());
    };

    // Handle escape key
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == Key::Escape {
            onclose.call(());
        }
    };

    rsx! {
        div {
            class: style::overlay,
            onclick: move |_| onclose.call(()),
            onkeydown: handle_keydown,

            div {
                class: stylance::classes!(style::dialog, variant_class),
                role: "alertdialog",
                aria_modal: "true",
                aria_labelledby: "alert-dialog-title",
                aria_describedby: "alert-dialog-description",
                onclick: move |e| e.stop_propagation(),

                // Icon
                div { class: style::icon_wrapper,
                    if let Some(custom_icon) = icon {
                        {custom_icon}
                    } else {
                        match variant {
                            AlertDialogVariant::Danger => rsx! {
                                svg {
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    line { x1: "15", y1: "9", x2: "9", y2: "15" }
                                    line { x1: "9", y1: "9", x2: "15", y2: "15" }
                                }
                            },
                            AlertDialogVariant::Warning => rsx! {
                                svg {
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                                    line { x1: "12", y1: "9", x2: "12", y2: "13" }
                                    line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
                                }
                            },
                            _ => rsx! {
                                svg {
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    line { x1: "12", y1: "16", x2: "12", y2: "12" }
                                    line { x1: "12", y1: "8", x2: "12.01", y2: "8" }
                                }
                            },
                        }
                    }
                }

                // Content
                div { class: style::content,
                    h2 {
                        id: "alert-dialog-title",
                        class: style::title,
                        "{title}"
                    }
                    p {
                        id: "alert-dialog-description",
                        class: style::description,
                        "{description}"
                    }
                }

                // Actions
                div { class: style::actions,
                    button {
                        class: style::cancel_button,
                        onclick: handle_cancel,
                        disabled: loading,
                        "{cancel_text}"
                    }
                    button {
                        class: stylance::classes!(style::confirm_button, variant_class),
                        onclick: handle_confirm,
                        disabled: loading,
                        if loading {
                            span { class: style::spinner }
                        }
                        "{confirm_text}"
                    }
                }
            }
        }
    }
}
