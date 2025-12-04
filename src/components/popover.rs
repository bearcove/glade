//! Popover component - click-triggered popup content

use dioxus::prelude::*;

stylance::import_style!(style, "popover.module.scss");

/// Position of the popover relative to trigger
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverPosition {
    /// Below the trigger, centered (default)
    #[default]
    Bottom,
    /// Above the trigger, centered
    Top,
    /// Left of the trigger
    Left,
    /// Right of the trigger
    Right,
    /// Below the trigger, aligned to start
    BottomStart,
    /// Below the trigger, aligned to end
    BottomEnd,
    /// Above the trigger, aligned to start
    TopStart,
    /// Above the trigger, aligned to end
    TopEnd,
}

/// Popover - click-triggered popup with arbitrary content
#[component]
pub fn Popover(
    /// Position relative to trigger
    #[props(default)]
    position: PopoverPosition,
    /// The trigger element
    trigger: Element,
    /// The popover content
    children: Element,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut container_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let position_class = match position {
        PopoverPosition::Bottom => style::bottom,
        PopoverPosition::Top => style::top,
        PopoverPosition::Left => style::left,
        PopoverPosition::Right => style::right,
        PopoverPosition::BottomStart => style::bottom_start,
        PopoverPosition::BottomEnd => style::bottom_end,
        PopoverPosition::TopStart => style::top_start,
        PopoverPosition::TopEnd => style::top_end,
    };

    // Close when clicking outside (client-side only)
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            use gloo_events::EventListener;
            use wasm_bindgen::JsCast;

            let window = web_sys::window();
            let Some(window) = window else { return };

            let listener = EventListener::new(&window, "click", move |event| {
                if !is_open() {
                    return;
                }

                let Some(target) = event.target() else {
                    return;
                };
                let Some(target) = target.dyn_ref::<web_sys::Element>() else {
                    return;
                };

                // Check if click is inside container
                if let Some(container) = container_ref() {
                    // Get raw element from container
                    if let Some(raw_elem) = container.downcast::<web_sys::Element>() {
                        if !raw_elem.contains(Some(target)) {
                            is_open.set(false);
                        }
                    }
                }
            });

            let _keep_alive = use_signal(|| listener);
        });
    }

    let popover_class = if is_open() {
        stylance::classes!(style::popover, position_class, style::open)
    } else {
        stylance::classes!(style::popover, position_class)
    };

    rsx! {
        div {
            class: style::popover_container,
            onmounted: move |e| container_ref.set(Some(e.data())),
            div {
                class: style::trigger,
                onclick: move |evt| {
                    evt.stop_propagation();
                    is_open.set(!is_open());
                },
                {trigger}
            }
            div {
                class: "{popover_class}",
                onclick: move |evt| evt.stop_propagation(),
                {children}
            }
        }
    }
}

/// PopoverContent - styled content wrapper with padding
#[component]
pub fn PopoverContent(children: Element) -> Element {
    rsx! {
        div { class: style::content, {children} }
    }
}
