//! Context menu component that appears on right-click

use dioxus::prelude::*;

stylance::import_style!(style, "context_menu.module.scss");

/// State shared between context menu components
#[derive(Clone, Copy)]
struct ContextMenuState {
    is_open: Signal<bool>,
    position: Signal<(i32, i32)>,
}

/// Context menu container that triggers on right-click.
///
/// Wrap any content with this component and it will show a menu
/// when right-clicking on that content.
#[component]
pub fn ContextMenu(children: Element) -> Element {
    let mut is_open = use_signal(|| false);
    let mut position = use_signal(|| (0_i32, 0_i32));

    // Provide state to children
    use_context_provider(|| ContextMenuState { is_open, position });

    // Close when clicking outside (client-side only)
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            use gloo_events::EventListener;

            let window = web_sys::window();
            let Some(window) = window else { return };

            let listener = EventListener::new(&window, "click", move |_| {
                if is_open() {
                    is_open.set(false);
                }
            });

            let _keep_alive = use_signal(|| listener);
        });
    }

    rsx! {
        div {
            class: style::context_menu,
            oncontextmenu: move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                let coords = evt.client_coordinates();
                position.set((coords.x as i32, coords.y as i32));
                is_open.set(true);
            },
            {children}
        }
    }
}

/// The content that can be right-clicked
#[component]
pub fn ContextMenuTrigger(children: Element) -> Element {
    rsx! {
        div { class: style::trigger, {children} }
    }
}

/// The menu that appears on right-click
#[component]
pub fn ContextMenuContent(children: Element) -> Element {
    let state = use_context::<ContextMenuState>();

    let menu_class = {
        let base = style::menu;
        if (state.is_open)() {
            stylance::classes!(base, style::open)
        } else {
            stylance::classes!(base)
        }
    };

    let menu_style = {
        let (x, y) = (state.position)();
        format!("left: {x}px; top: {y}px;")
    };

    rsx! {
        div {
            class: "{menu_class}",
            style: "{menu_style}",
            onclick: move |evt| evt.stop_propagation(),
            {children}
        }
    }
}

/// A menu item in the context menu
#[component]
pub fn ContextMenuItem(
    /// Whether the item is disabled
    #[props(default = false)]
    disabled: bool,
    /// Click handler
    onclick: Option<EventHandler<MouseEvent>>,
    /// Item content
    children: Element,
) -> Element {
    let mut state = use_context::<ContextMenuState>();

    rsx! {
        button {
            class: style::item,
            disabled: disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
                state.is_open.set(false);
            },
            {children}
        }
    }
}

/// A divider between menu items
#[component]
pub fn ContextMenuDivider() -> Element {
    rsx! {
        div { class: style::divider }
    }
}
