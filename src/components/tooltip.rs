//! Tooltip component - hover-triggered informational popup

use dioxus::prelude::*;

stylance::import_style!(style, "tooltip.module.scss");

/// Position of the tooltip relative to the trigger element
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    /// Above the trigger element (default)
    #[default]
    Top,
    /// Below the trigger element
    Bottom,
    /// Left of the trigger element
    Left,
    /// Right of the trigger element
    Right,
}

/// Tooltip - shows informational text on hover
#[component]
pub fn Tooltip(
    /// The tooltip text to display
    #[props(into)]
    text: String,
    /// Position relative to the trigger element
    #[props(default = TooltipPosition::Top)]
    position: TooltipPosition,
    /// Delay in milliseconds before showing (default: 200)
    #[props(default = 200)]
    delay: u32,
    /// The trigger element (what gets hovered)
    children: Element,
) -> Element {
    #[allow(unused_mut, reason = "wasm-only")]
    let mut visible = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    let (on_mouse_enter, on_mouse_leave) = {
        use gloo_timers::callback::Timeout;

        // store timeout so we can cancel when leaving
        let mut timeout_handle = use_signal::<Option<Timeout>>(|| None);

        let enter = move |_| {
            let handle = Timeout::new(delay, {
                let mut visible = visible.clone();
                move || visible.set(true)
            });
            timeout_handle.set(Some(handle));
        };

        let leave = move |_| {
            if let Some(handle) = timeout_handle.take() {
                handle.cancel();
            }
            visible.set(false);
        };

        (enter, leave)
    };

    #[cfg(not(target_arch = "wasm32"))]
    let (on_mouse_enter, on_mouse_leave) = (|_| {}, |_| {});

    let position_class = match position {
        TooltipPosition::Top => style::top,
        TooltipPosition::Bottom => style::bottom,
        TooltipPosition::Left => style::left,
        TooltipPosition::Right => style::right,
    };

    let tooltip_class = if visible() {
        stylance::classes!(style::tooltip, position_class, style::visible)
    } else {
        stylance::classes!(style::tooltip, position_class)
    };

    rsx! {
        div {
            class: style::tooltip_wrapper,
            onmouseenter: on_mouse_enter,
            onmouseleave: on_mouse_leave,

            {children}
            div {
                class: tooltip_class,
                role: "tooltip",
                {text}
            }
        }
    }
}
