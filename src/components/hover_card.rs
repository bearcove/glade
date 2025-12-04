//! HoverCard component for rich content on hover

use dioxus::prelude::*;

stylance::import_style!(style, "hover_card.module.scss");

/// Position of the hover card relative to trigger
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverCardPosition {
    /// Show above the trigger
    Top,
    /// Show below the trigger (default)
    #[default]
    Bottom,
    /// Show to the left of the trigger
    Left,
    /// Show to the right of the trigger
    Right,
}

/// A card that appears on hover, showing rich content (more than a tooltip)
#[component]
pub fn HoverCard(
    /// The trigger element that activates the hover card
    trigger: Element,
    /// The content to show in the hover card
    children: Element,
    /// Position relative to trigger
    #[props(default)]
    position: HoverCardPosition,
    /// Delay before showing (ms)
    #[props(default = 200)]
    open_delay: u32,
    /// Delay before hiding (ms)
    #[props(default = 100)]
    close_delay: u32,
    /// Additional CSS class for the card
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut intent_to_open = use_signal(|| false);
    let mut intent_to_close = use_signal(|| false);

    let position_class = match position {
        HoverCardPosition::Top => style::top,
        HoverCardPosition::Bottom => style::bottom,
        HoverCardPosition::Left => style::left,
        HoverCardPosition::Right => style::right,
    };

    // Handle open delay using gloo timers (WASM compatible)
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            if intent_to_open() && !is_open() {
                let delay = open_delay;
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(delay).await;
                    if intent_to_open() {
                        is_open.set(true);
                    }
                });
            }
        });

        // Handle close delay
        use_effect(move || {
            if intent_to_close() && is_open() {
                let delay = close_delay;
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(delay).await;
                    if intent_to_close() {
                        is_open.set(false);
                    }
                });
            }
        });
    }

    // For SSR/non-WASM, just open/close immediately
    #[cfg(not(target_arch = "wasm32"))]
    {
        use_effect(move || {
            if intent_to_open() && !is_open() {
                is_open.set(true);
            }
        });
        use_effect(move || {
            if intent_to_close() && is_open() {
                is_open.set(false);
            }
        });
    }

    let handle_enter = move |_| {
        intent_to_close.set(false);
        intent_to_open.set(true);
    };

    let handle_leave = move |_| {
        intent_to_open.set(false);
        intent_to_close.set(true);
    };

    rsx! {
        div {
            class: style::hover_card_wrapper,
            onmouseenter: handle_enter,
            onmouseleave: handle_leave,

            // Trigger
            div { class: style::trigger,
                {trigger}
            }

            // Card content
            div {
                class: stylance::classes!(
                    style::card,
                    position_class,
                    if is_open() { style::open } else { "" },
                    class.as_deref().unwrap_or("")
                ),
                role: "tooltip",
                onmouseenter: handle_enter,
                onmouseleave: handle_leave,

                {children}
            }
        }
    }
}

/// Pre-styled hover card for user profile previews
#[component]
pub fn ProfileHoverCard(
    /// The trigger element
    trigger: Element,
    /// User avatar URL
    #[props(optional, into)]
    avatar: Option<String>,
    /// User name
    #[props(into)]
    name: String,
    /// Username/handle
    #[props(optional, into)]
    username: Option<String>,
    /// User bio/description
    #[props(optional, into)]
    bio: Option<String>,
    /// Position
    #[props(default)]
    position: HoverCardPosition,
    /// Additional content (stats, buttons, etc.)
    #[props(optional)]
    footer: Option<Element>,
) -> Element {
    rsx! {
        HoverCard {
            trigger,
            position,
            div { class: style::profile_card,
                // Header with avatar
                div { class: style::profile_header,
                    if let Some(avatar_url) = avatar {
                        img {
                            class: style::profile_avatar,
                            src: "{avatar_url}",
                            alt: "{name}",
                        }
                    } else {
                        div { class: style::profile_avatar_placeholder,
                            {name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                        }
                    }
                    div { class: style::profile_info,
                        span { class: style::profile_name, "{name}" }
                        if let Some(handle) = username {
                            span { class: style::profile_username, "@{handle}" }
                        }
                    }
                }

                // Bio
                if let Some(bio_text) = bio {
                    p { class: style::profile_bio, "{bio_text}" }
                }

                // Footer
                if let Some(footer_content) = footer {
                    div { class: style::profile_footer,
                        {footer_content}
                    }
                }
            }
        }
    }
}
