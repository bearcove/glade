//! Attachment chip component for displaying file attachments

use dioxus::prelude::*;

use crate::{IconFileText, IconX};

stylance::import_style!(style, "attachment_chip.module.scss");

/// Type of attachment
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentType {
    /// Generic file
    #[default]
    File,
    /// Image file
    Image,
    /// Code/context reference
    Context,
}

/// Attachment chip showing file info with remove button
#[component]
pub fn AttachmentChip(
    /// File name
    name: String,
    /// Optional file size (formatted string like "2.5 KB")
    size: Option<String>,
    /// Type of attachment
    #[props(default)]
    attachment_type: AttachmentType,
    /// Called when remove button is clicked
    on_remove: Option<EventHandler<()>>,
    /// Whether the chip is disabled
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let type_class = match attachment_type {
        AttachmentType::File => "",
        AttachmentType::Image => style::image,
        AttachmentType::Context => style::context,
    };

    rsx! {
        span { class: stylance::classes!(style::chip, type_class, if disabled { style::disabled } else { "" }),
            span { class: style::icon, IconFileText {} }
            span { class: style::name, title: "{name}", "{name}" }
            if let Some(s) = size {
                span { class: style::size, "{s}" }
            }
            if let Some(cb) = on_remove {
                button {
                    r#type: "button",
                    class: style::remove,
                    disabled: disabled,
                    onclick: move |_| cb.call(()),
                    IconX {}
                }
            }
        }
    }
}

/// Container for multiple attachment chips
#[component]
pub fn AttachmentList(children: Element) -> Element {
    rsx! {
        div { class: style::list, {children} }
    }
}
