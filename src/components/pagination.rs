//! Pagination component for navigating through pages of content

use dioxus::prelude::*;

stylance::import_style!(style, "pagination.module.scss");

/// Size variants for pagination
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PaginationSize {
    /// Small pagination buttons
    Small,
    /// Medium pagination buttons (default)
    #[default]
    Medium,
    /// Large pagination buttons
    Large,
}

/// Pagination component for navigating pages
#[component]
pub fn Pagination(
    /// Current page (1-indexed)
    current: usize,
    /// Total number of pages
    total: usize,
    /// Callback when page changes
    on_change: EventHandler<usize>,
    /// Number of pages to show on each side of current
    #[props(default = 1)]
    siblings: usize,
    /// Number of pages to show at start/end
    #[props(default = 1)]
    boundaries: usize,
    /// Size variant
    #[props(default)]
    size: PaginationSize,
    /// Show previous/next buttons
    #[props(default = true)]
    show_controls: bool,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let size_class = match size {
        PaginationSize::Small => style::small,
        PaginationSize::Medium => "",
        PaginationSize::Large => style::large,
    };

    // Calculate which pages to show
    let pages = compute_pages(current, total, siblings, boundaries);

    rsx! {
        nav {
            class: stylance::classes!(style::pagination, size_class, if disabled { style::disabled } else { "" }),
            role: "navigation",
            aria_label: "Pagination",

            if show_controls {
                button {
                    class: stylance::classes!(style::page_button, style::control),
                    disabled: disabled || current <= 1,
                    onclick: move |_| {
                        if current > 1 {
                            on_change.call(current - 1);
                        }
                    },
                    aria_label: "Previous page",
                    // Chevron left
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
            }

            for item in pages {
                match item {
                    PageItem::Page(page) => rsx! {
                        button {
                            key: "page-{page}",
                            class: stylance::classes!(
                                style::page_button,
                                if page == current { style::active } else { "" }
                            ),
                            disabled: disabled,
                            onclick: move |_| on_change.call(page),
                            aria_current: if page == current { "page" } else { "" },
                            "{page}"
                        }
                    },
                    PageItem::Ellipsis(key) => rsx! {
                        span {
                            key: "ellipsis-{key}",
                            class: style::ellipsis,
                            "…"
                        }
                    },
                }
            }

            if show_controls {
                button {
                    class: stylance::classes!(style::page_button, style::control),
                    disabled: disabled || current >= total,
                    onclick: move |_| {
                        if current < total {
                            on_change.call(current + 1);
                        }
                    },
                    aria_label: "Next page",
                    // Chevron right
                    svg {
                        width: "16",
                        height: "16",
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
        }
    }
}

#[derive(Clone, Copy)]
enum PageItem {
    Page(usize),
    Ellipsis(&'static str),
}

fn compute_pages(current: usize, total: usize, siblings: usize, boundaries: usize) -> Vec<PageItem> {
    if total <= 0 {
        return vec![];
    }

    let mut items = Vec::new();

    // Calculate ranges
    let left_sibling_start = current.saturating_sub(siblings).max(1);
    let right_sibling_end = (current + siblings).min(total);

    let show_left_ellipsis = left_sibling_start > boundaries + 2;
    let show_right_ellipsis = right_sibling_end < total.saturating_sub(boundaries + 1);

    // Add boundary pages at start
    for page in 1..=boundaries.min(total) {
        if page < left_sibling_start {
            items.push(PageItem::Page(page));
        }
    }

    // Add left ellipsis
    if show_left_ellipsis {
        items.push(PageItem::Ellipsis("left"));
    } else if boundaries > 0 && boundaries < left_sibling_start {
        // Fill in missing pages between boundaries and siblings
        for page in (boundaries + 1)..left_sibling_start {
            items.push(PageItem::Page(page));
        }
    }

    // Add sibling pages and current
    for page in left_sibling_start..=right_sibling_end {
        items.push(PageItem::Page(page));
    }

    // Add right ellipsis
    if show_right_ellipsis {
        items.push(PageItem::Ellipsis("right"));
    } else if boundaries > 0 && right_sibling_end < total.saturating_sub(boundaries - 1) {
        // Fill in missing pages between siblings and end boundaries
        for page in (right_sibling_end + 1)..=(total.saturating_sub(boundaries)) {
            items.push(PageItem::Page(page));
        }
    }

    // Add boundary pages at end
    for page in (total.saturating_sub(boundaries - 1))..=total {
        if page > right_sibling_end {
            items.push(PageItem::Page(page));
        }
    }

    items
}

/// Simple pagination showing only previous/next buttons with page info
#[component]
pub fn SimplePagination(
    /// Current page (1-indexed)
    current: usize,
    /// Total number of pages
    total: usize,
    /// Callback when page changes
    on_change: EventHandler<usize>,
    /// Size variant
    #[props(default)]
    size: PaginationSize,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let size_class = match size {
        PaginationSize::Small => style::small,
        PaginationSize::Medium => "",
        PaginationSize::Large => style::large,
    };

    rsx! {
        nav {
            class: stylance::classes!(style::pagination, style::simple, size_class, if disabled { style::disabled } else { "" }),
            role: "navigation",
            aria_label: "Pagination",

            button {
                class: stylance::classes!(style::page_button, style::control),
                disabled: disabled || current <= 1,
                onclick: move |_| {
                    if current > 1 {
                        on_change.call(current - 1);
                    }
                },
                "Previous"
            }

            span { class: style::page_info,
                "Page {current} of {total}"
            }

            button {
                class: stylance::classes!(style::page_button, style::control),
                disabled: disabled || current >= total,
                onclick: move |_| {
                    if current < total {
                        on_change.call(current + 1);
                    }
                },
                "Next"
            }
        }
    }
}
