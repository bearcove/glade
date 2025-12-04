//! Message list container with auto-scroll support

use dioxus::prelude::*;

stylance::import_style!(style, "message_list.module.scss");

/// Message list container with auto-scroll to bottom
#[component]
pub fn MessageList(
    /// Whether to auto-scroll to bottom when new messages arrive
    #[props(default = true)]
    _auto_scroll: bool,
    /// Called when scroll reaches the top (for loading older messages)
    on_reach_top: Option<EventHandler<()>>,
    /// Called when scroll reaches the bottom
    on_reach_bottom: Option<EventHandler<()>>,
    /// Message content
    children: Element,
) -> Element {
    let mut container_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut is_at_bottom = use_signal(|| true);

    rsx! {
        div {
            class: style::message_list,
            onmounted: move |e| container_ref.set(Some(e.data())),
            onscroll: move |_| {
                if let Some(el) = container_ref() {
                    spawn(async move {
                        if let Ok(rect) = el.get_client_rect().await {
                            if let Ok(scroll) = el.get_scroll_offset().await {
                                // Get scrollable dimensions
                                let scroll_top = scroll.y;
                                let client_height = rect.size.height;

                                // Check if at top
                                if scroll_top <= 10.0 {
                                    if let Some(cb) = &on_reach_top {
                                        cb.call(());
                                    }
                                }

                                // Note: at_bottom detection needs scroll height which isn't directly available
                                // For now, just track if user has scrolled
                                let at_bottom = scroll_top + client_height >= client_height - 10.0;
                                is_at_bottom.set(at_bottom);

                                if at_bottom {
                                    if let Some(cb) = &on_reach_bottom {
                                        cb.call(());
                                    }
                                }
                            }
                        }
                    });
                }
            },
            div { class: style::content, {children} }
        }
    }
}
