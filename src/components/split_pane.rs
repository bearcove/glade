//! Split pane / resizable panels component

use dioxus::prelude::*;

stylance::import_style!(style, "split_pane.module.scss");

/// Direction of the split
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    /// Horizontal split (panels side by side)
    #[default]
    Horizontal,
    /// Vertical split (panels stacked)
    Vertical,
}

/// Resizable split pane component
#[component]
pub fn SplitPane(
    /// Split direction
    #[props(default)]
    direction: SplitDirection,
    /// Initial size of the first panel (CSS value or percentage)
    #[props(default = "50%".to_string())]
    initial_size: String,
    /// Minimum size of the first panel (CSS value)
    #[props(default = "100px".to_string())]
    min_size: String,
    /// Maximum size of the first panel (CSS value)
    max_size: Option<String>,
    /// First panel content
    first: Option<Element>,
    /// Second panel content
    second: Option<Element>,
) -> Element {
    let mut size = use_signal(move || initial_size.clone());
    let mut is_dragging = use_signal(|| false);
    let mut container_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let direction_class = match direction {
        SplitDirection::Horizontal => style::horizontal,
        SplitDirection::Vertical => style::vertical,
    };

    let first_style = {
        let s = size();
        let min = min_size.clone();
        let max = max_size.clone();
        match direction {
            SplitDirection::Horizontal => format!(
                "width: {s}; min-width: {min};{}",
                max.as_ref()
                    .map(|m| format!(" max-width: {m};"))
                    .unwrap_or_default()
            ),
            SplitDirection::Vertical => format!(
                "height: {s}; min-height: {min};{}",
                max.as_ref()
                    .map(|m| format!(" max-height: {m};"))
                    .unwrap_or_default()
            ),
        }
    };

    let container_class = stylance::classes!(
        style::split_pane,
        direction_class,
        if is_dragging() { style::dragging } else { "" }
    );

    rsx! {
        div {
            class: "{container_class}",
            onmounted: move |e| container_ref.set(Some(e.data())),
            onmousemove: move |evt| {
                if !is_dragging() {
                    return;
                }

                if let Some(container) = container_ref() {
                    spawn(async move {
                        if let Ok(rect) = container.get_client_rect().await {
                            let coords = evt.client_coordinates();
                            let new_size = match direction {
                                SplitDirection::Horizontal => {
                                    let x = coords.x - rect.origin.x;
                                    let percentage = (x / rect.size.width) * 100.0;
                                    let clamped = percentage.clamp(10.0, 90.0);
                                    format!("{clamped}%")
                                }
                                SplitDirection::Vertical => {
                                    let y = coords.y - rect.origin.y;
                                    let percentage = (y / rect.size.height) * 100.0;
                                    let clamped = percentage.clamp(10.0, 90.0);
                                    format!("{clamped}%")
                                }
                            };
                            size.set(new_size);
                        }
                    });
                }
            },
            onmouseup: move |_| {
                is_dragging.set(false);
            },
            onmouseleave: move |_| {
                is_dragging.set(false);
            },
            div { class: style::panel, style: "{first_style}", {first} }
            div {
                class: style::divider,
                onmousedown: move |_| {
                    is_dragging.set(true);
                },
                div { class: style::divider_handle }
            }
            div { class: style::panel, {second} }
        }
    }
}

/// A panel within a split pane (for more complex layouts)
#[component]
pub fn Panel(children: Element) -> Element {
    rsx! {
        div { class: style::panel_content, {children} }
    }
}
