//! Carousel component for sliding through content

use dioxus::prelude::*;

stylance::import_style!(style, "carousel.module.scss");

/// Carousel navigation style
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CarouselNavigation {
    /// Show prev/next arrow buttons (default)
    #[default]
    Arrows,
    /// Show dot indicators only
    Dots,
    /// Show both arrows and dots
    Both,
    /// No navigation controls
    None,
}

/// Carousel component for cycling through slides
#[component]
pub fn Carousel(
    /// Slide content
    children: Element,
    /// Number of slides (must match children)
    slides: usize,
    /// Initial slide index
    #[props(default = 0)]
    initial: usize,
    /// Navigation style
    #[props(default)]
    navigation: CarouselNavigation,
    /// Auto-play interval in ms (0 = disabled)
    #[props(default = 0)]
    autoplay: u32,
    /// Loop back to start when at end
    #[props(default = true)]
    infinite: bool,
    /// Callback when slide changes
    #[props(optional)]
    on_change: Option<EventHandler<usize>>,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let mut current = use_signal(|| initial.min(slides.saturating_sub(1)));

    let mut go_to = move |index: usize| {
        let new_index = if infinite {
            index % slides
        } else {
            index.min(slides.saturating_sub(1))
        };
        current.set(new_index);
        if let Some(handler) = &on_change {
            handler.call(new_index);
        }
    };

    let go_prev = move |_| {
        let cur = current();
        if cur == 0 {
            if infinite {
                go_to(slides.saturating_sub(1));
            }
        } else {
            go_to(cur - 1);
        }
    };

    let go_next = move |_| {
        let cur = current();
        if cur >= slides.saturating_sub(1) {
            if infinite {
                go_to(0);
            }
        } else {
            go_to(cur + 1);
        }
    };

    // Auto-play effect (WASM only for now)
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if autoplay > 0 {
            spawn(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(autoplay).await;
                    let cur = current();
                    if cur >= slides.saturating_sub(1) {
                        if infinite {
                            current.set(0);
                        }
                    } else {
                        current.set(cur + 1);
                    }
                }
            });
        }
    });

    // Suppress unused variable warning in non-WASM
    #[cfg(not(target_arch = "wasm32"))]
    let _ = autoplay;

    let show_arrows = matches!(navigation, CarouselNavigation::Arrows | CarouselNavigation::Both);
    let show_dots = matches!(navigation, CarouselNavigation::Dots | CarouselNavigation::Both);

    rsx! {
        div {
            class: stylance::classes!(style::carousel, class.as_deref().unwrap_or("")),
            role: "region",
            aria_roledescription: "carousel",
            aria_label: "Image carousel",

            // Slides container
            div { class: style::viewport,
                div {
                    class: style::track,
                    style: format!("transform: translateX(-{}%);", current() * 100),
                    {children}
                }
            }

            // Arrow navigation
            if show_arrows {
                button {
                    class: stylance::classes!(style::arrow, style::arrow_prev),
                    onclick: go_prev,
                    disabled: !infinite && current() == 0,
                    aria_label: "Previous slide",
                    svg {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                button {
                    class: stylance::classes!(style::arrow, style::arrow_next),
                    onclick: go_next,
                    disabled: !infinite && current() >= slides.saturating_sub(1),
                    aria_label: "Next slide",
                    svg {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }

            // Dot navigation
            if show_dots {
                div {
                    class: style::dots,
                    role: "tablist",
                    for i in 0..slides {
                        button {
                            class: stylance::classes!(
                                style::dot,
                                if current() == i { style::active } else { "" }
                            ),
                            onclick: move |_| go_to(i),
                            role: "tab",
                            aria_selected: current() == i,
                            aria_label: format!("Go to slide {}", i + 1),
                        }
                    }
                }
            }
        }
    }
}

/// Individual slide wrapper
#[component]
pub fn CarouselSlide(
    /// Slide content
    children: Element,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    rsx! {
        div {
            class: stylance::classes!(style::slide, class.as_deref().unwrap_or("")),
            role: "tabpanel",
            {children}
        }
    }
}
