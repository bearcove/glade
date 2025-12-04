//! `TodoList` component - a list with dashed circles that fill when checked

use dioxus::prelude::*;

stylance::import_style!(style, "todo_list.module.scss");

/// Size variants for todo items
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TodoSize {
    /// Small todo item
    Small,
    /// Medium todo item (default)
    #[default]
    Medium,
    /// Large todo item
    Large,
}

/// TodoList - container for todo items
#[component]
pub fn TodoList(children: Element) -> Element {
    rsx! {
        ul { class: style::todo_list, {children} }
    }
}

/// TodoItem - a single todo item with a dashed circle checkbox
#[component]
pub fn TodoItem(
    /// Whether the item is checked/completed
    #[props(default = false)]
    checked: bool,
    /// Size of the checkbox
    #[props(default)]
    size: TodoSize,
    /// Whether the item is disabled
    #[props(default = false)]
    disabled: bool,
    /// Called when the item is clicked
    onchange: Option<EventHandler<()>>,
    /// The todo item content
    children: Element,
) -> Element {
    let size_class = match size {
        TodoSize::Small => style::small,
        TodoSize::Medium => "",
        TodoSize::Large => style::large,
    };

    let disabled_class = if disabled { style::disabled } else { "" };
    let checked_class = if checked { style::checked } else { "" };
    let content_checked_class = if checked { style::content_checked } else { "" };

    rsx! {
        li { class: stylance::classes!(style::todo_item, size_class, disabled_class),
            button {
                class: stylance::classes!(style::checkbox, checked_class),
                role: "checkbox",
                aria_checked: "{checked}",
                disabled: disabled,
                onclick: move |_| {
                    if let Some(handler) = &onchange {
                        handler.call(());
                    }
                },
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    // Dashed circle
                    circle {
                        cx: "12",
                        cy: "12",
                        r: "10",
                        class: style::circle,
                    }
                    // Checkmark (visible when checked)
                    path {
                        d: "M8 12.5L10.5 15L16 9",
                        class: style::checkmark,
                    }
                }
            }
            span { class: stylance::classes!(style::content, content_checked_class), {children} }
        }
    }
}
