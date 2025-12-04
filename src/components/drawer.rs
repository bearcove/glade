//! Drawer/SlideOver component - side panel that slides in

use dioxus::prelude::*;

use crate::IconX;

stylance::import_style!(style, "drawer.module.scss");

/// Position of the drawer
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPosition {
    /// Slides in from the right (default)
    #[default]
    Right,
    /// Slides in from the left
    Left,
}

/// Size of the drawer
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerSize {
    /// Small drawer width
    Small,
    /// Medium drawer width (default)
    #[default]
    Medium,
    /// Large drawer width
    Large,
    /// Full-screen drawer
    Full,
}

/// Drawer - side panel that slides in from left or right
#[component]
pub fn Drawer(
    /// Whether the drawer is open
    open: Signal<bool>,
    /// Position (left or right)
    #[props(default)]
    position: DrawerPosition,
    /// Size of the drawer
    #[props(default)]
    size: DrawerSize,
    /// Called when drawer should close (clicking overlay or close button)
    onclose: Option<EventHandler<()>>,
    /// Drawer content
    children: Element,
) -> Element {
    let position_class = match position {
        DrawerPosition::Right => style::right,
        DrawerPosition::Left => style::left,
    };

    let size_class = match size {
        DrawerSize::Small => style::small,
        DrawerSize::Medium => style::medium,
        DrawerSize::Large => style::large,
        DrawerSize::Full => style::full,
    };

    let is_open = open();
    let overlay_class = if is_open {
        stylance::classes!(style::overlay, style::overlay_visible)
    } else {
        style::overlay.to_string()
    };

    let drawer_class = if is_open {
        stylance::classes!(style::drawer, position_class, size_class, style::drawer_open)
    } else {
        stylance::classes!(style::drawer, position_class, size_class)
    };

    rsx! {
        div {
            class: style::drawer_container,
            style: if is_open { "display: block" } else { "display: none" },
            div {
                class: "{overlay_class}",
                onclick: move |_| {
                    if let Some(handler) = &onclose {
                        handler.call(());
                    }
                },
            }
            div {
                class: "{drawer_class}",
                role: "dialog",
                aria_modal: "true",
                {children}
            }
        }
    }
}

/// DrawerHeader - header section with title and optional close button
#[component]
pub fn DrawerHeader(
    /// Called when close button is clicked
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    rsx! {
        div { class: style::header,
            div { class: style::header_content, {children} }
            if let Some(handler) = onclose {
                button {
                    class: style::close_button,
                    aria_label: "Close drawer",
                    onclick: move |_| handler.call(()),
                    IconX {}
                }
            }
        }
    }
}

/// DrawerBody - main content area
#[component]
pub fn DrawerBody(children: Element) -> Element {
    rsx! {
        div { class: style::body, {children} }
    }
}

/// DrawerFooter - footer section for actions
#[component]
pub fn DrawerFooter(children: Element) -> Element {
    rsx! {
        div { class: style::footer, {children} }
    }
}
