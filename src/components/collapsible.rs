//! Collapsible component for expandable/collapsible content

use dioxus::prelude::*;

stylance::import_style!(style, "collapsible.module.scss");

/// A simple collapsible panel (simpler than Accordion - for single items)
#[component]
pub fn Collapsible(
    /// Whether the collapsible is open
    #[props(default = false)]
    open: bool,
    /// Callback when open state changes
    #[props(optional)]
    on_toggle: Option<EventHandler<bool>>,
    /// The trigger element (clickable header)
    trigger: Element,
    /// The collapsible content
    children: Element,
    /// Whether the collapsible is disabled
    #[props(default = false)]
    disabled: bool,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let mut is_open = use_signal(|| open);

    // Sync with controlled prop
    use_effect(move || {
        is_open.set(open);
    });

    let handle_toggle = move |_: MouseEvent| {
        if !disabled {
            let new_state = !is_open();
            is_open.set(new_state);
            if let Some(handler) = &on_toggle {
                handler.call(new_state);
            }
        }
    };

    rsx! {
        div {
            class: stylance::classes!(
                style::collapsible,
                if is_open() { style::open } else { "" },
                if disabled { style::disabled } else { "" },
                class.as_deref().unwrap_or("")
            ),
            "data-state": if is_open() { "open" } else { "closed" },

            div {
                class: style::trigger,
                role: "button",
                tabindex: if disabled { "-1" } else { "0" },
                aria_expanded: is_open(),
                aria_disabled: disabled,
                onclick: handle_toggle,
                onkeydown: move |e: KeyboardEvent| {
                    if !disabled && (e.key() == Key::Enter || e.key() == Key::Character(" ".to_string())) {
                        e.prevent_default();
                        // Toggle on keyboard
                        let new_state = !is_open();
                        is_open.set(new_state);
                        if let Some(handler) = &on_toggle {
                            handler.call(new_state);
                        }
                    }
                },

                {trigger}

                // Chevron icon
                span { class: style::icon,
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "6 9 12 15 18 9" }
                    }
                }
            }

            div {
                class: style::content_wrapper,
                div {
                    class: style::content,
                    role: "region",
                    {children}
                }
            }
        }
    }
}

/// A collapsible with a default styled trigger
#[component]
pub fn CollapsiblePanel(
    /// Title for the panel
    #[props(into)]
    title: String,
    /// Whether the collapsible is open
    #[props(default = false)]
    open: bool,
    /// Callback when open state changes
    #[props(optional)]
    on_toggle: Option<EventHandler<bool>>,
    /// The collapsible content
    children: Element,
    /// Optional subtitle/description
    #[props(optional, into)]
    subtitle: Option<String>,
    /// Whether the collapsible is disabled
    #[props(default = false)]
    disabled: bool,
) -> Element {
    rsx! {
        Collapsible {
            open,
            on_toggle,
            disabled,
            trigger: rsx! {
                div { class: style::panel_header,
                    span { class: style::panel_title, "{title}" }
                    if let Some(sub) = subtitle {
                        span { class: style::panel_subtitle, "{sub}" }
                    }
                }
            },
            {children}
        }
    }
}
