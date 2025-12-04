//! Modal dialog component for overlaid content

use dioxus::prelude::*;

use super::icon_button::IconButton;
use super::icons::IconX;

stylance::import_style!(style, "modal.module.scss");

/// Size variants for Modal
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ModalSize {
    /// Small modal (400px max width)
    Small,
    /// Medium modal (600px max width, default)
    #[default]
    Medium,
    /// Large modal (800px max width)
    Large,
    /// Full screen modal
    Full,
}

#[component]
pub fn Modal(
    #[props(default = false)] open: bool,
    #[props(default)] size: ModalSize,
    onclose: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let size_class = match size {
        ModalSize::Small => style::small,
        ModalSize::Medium => "",
        ModalSize::Large => style::large,
        ModalSize::Full => style::full,
    };

    if !open {
        return rsx! {};
    }

    rsx! {
        div { class: style::overlay,
            onclick: move |evt| {
                if let Some(handler) = &onclose {
                    handler.call(evt);
                }
            },
            div {
                class: stylance::classes!(style::modal, size_class),
                onclick: move |evt| evt.stop_propagation(),
                {children}
            }
        }
    }
}

#[component]
pub fn ModalHeader(onclose: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
    rsx! {
        div { class: style::header,
            div { class: style::title,
                {children}
            }
            if let Some(handler) = onclose {
                IconButton {
                    aria_label: "Close".to_string(),
                    onclick: move |evt| handler.call(evt),
                    IconX {}
                }
            }
        }
    }
}

#[component]
pub fn ModalBody(children: Element) -> Element {
    rsx! {
        div { class: style::body,
            {children}
        }
    }
}

#[component]
pub fn ModalFooter(children: Element) -> Element {
    rsx! {
        div { class: style::footer,
            {children}
        }
    }
}
