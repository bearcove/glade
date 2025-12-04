//! Dropdown menu component for contextual actions

use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_events::EventListener;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

stylance::import_style!(style, "dropdown.module.scss");

/// Alignment options for dropdown menu positioning
#[derive(Clone, Copy, PartialEq, Default)]
pub enum DropdownAlign {
    /// Align to the start (left in LTR)
    #[default]
    Start,
    /// Align to the end (right in LTR)
    End,
}

/// Dropdown container that manages its own open/close state.
///
/// Automatically closes when clicking outside the dropdown.
/// Use `DropdownTrigger` to toggle, and items will auto-close on click.
#[component]
pub fn Dropdown(#[props(default)] align: DropdownAlign, children: Element) -> Element {
    #[allow(unused_mut, reason = "only used in wasm")]
    let mut is_open = use_signal(|| false);

    let align_class = match align {
        DropdownAlign::Start => style::start,
        DropdownAlign::End => style::end,
    };

    let open_class = if is_open() { style::open } else { "" };

    // Set up document click listener for outside clicks (WASM only)
    #[cfg(target_arch = "wasm32")]
    let _listener = use_signal(move || {
        let document = web_sys::window().and_then(|w| w.document());

        if let Some(document) = document {
            Some(EventListener::new(&document, "click", move |event| {
                // Only process if dropdown is open
                if !*is_open.read() {
                    return;
                }

                if let Some(target) = event.target() {
                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                        // Check if click is outside any dropdown container
                        if element
                            .closest("[data-dropdown-container]")
                            .ok()
                            .flatten()
                            .is_none()
                        {
                            is_open.set(false);
                        }
                    }
                }
            }))
        } else {
            None
        }
    });

    // Provide the open state to children via context
    use_context_provider(move || DropdownState { is_open });

    rsx! {
        div {
            class: stylance::classes!(style::dropdown, align_class, open_class),
            "data-dropdown-container": "true",
            {children}
        }
    }
}

/// Internal state shared between Dropdown and its children
#[derive(Clone, Copy)]
struct DropdownState {
    is_open: Signal<bool>,
}

#[component]
pub fn DropdownTrigger(children: Element) -> Element {
    let state = try_use_context::<DropdownState>();

    rsx! {
        div {
            class: style::trigger,
            onclick: move |evt| {
                evt.stop_propagation();
                if let Some(mut state) = state {
                    let current = *state.is_open.read();
                    state.is_open.set(!current);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn DropdownMenu(children: Element) -> Element {
    rsx! {
        div {
            class: style::menu,
            onclick: move |evt| {
                // Prevent clicks inside menu from bubbling to document
                evt.stop_propagation();
            },
            {children}
        }
    }
}

#[component]
pub fn DropdownItem(
    #[props(default = false)] disabled: bool,
    #[props(default = false)] keep_open: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let state = try_use_context::<DropdownState>();

    rsx! {
        button {
            class: style::item,
            disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
                // Close dropdown after click unless keep_open is true
                if !keep_open {
                    if let Some(mut state) = state {
                        state.is_open.set(false);
                    }
                }
            },
            {children}
        }
    }
}

/// A visual divider between dropdown menu items
#[component]
pub fn DropdownDivider() -> Element {
    rsx! {
        div { class: style::divider }
    }
}
