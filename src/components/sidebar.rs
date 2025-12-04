use dioxus::prelude::*;

stylance::import_style!(style, "sidebar.module.scss");

#[component]
pub fn Sidebar(#[props(default = false)] collapsed: bool, children: Element) -> Element {
    let collapsed_class = if collapsed { style::collapsed } else { "" };

    rsx! {
        aside { class: stylance::classes!(style::sidebar, collapsed_class),
            {children}
        }
    }
}

#[component]
pub fn SidebarHeader(children: Element) -> Element {
    rsx! {
        div { class: style::header,
            {children}
        }
    }
}

#[component]
pub fn SidebarNav(children: Element) -> Element {
    rsx! {
        nav { class: style::nav,
            {children}
        }
    }
}

#[component]
pub fn SidebarSection(#[props(default)] title: String, children: Element) -> Element {
    rsx! {
        div { class: style::section,
            if !title.is_empty() {
                div { class: style::section_title, "{title}" }
            }
            {children}
        }
    }
}

#[component]
pub fn SidebarItem(
    #[props(default = false)] active: bool,
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let active_class = if active { style::item_active } else { "" };

    rsx! {
        button {
            class: stylance::classes!(style::item, active_class),
            disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn SidebarFooter(children: Element) -> Element {
    rsx! {
        div { class: style::footer,
            {children}
        }
    }
}

/// SidebarGroup - a group with a heading and nested items
#[component]
pub fn SidebarGroup(children: Element) -> Element {
    rsx! {
        div { class: style::group,
            {children}
        }
    }
}

/// SidebarGroupItems - container for nested items within a group
#[component]
pub fn SidebarGroupItems(children: Element) -> Element {
    rsx! {
        div { class: style::group_items,
            {children}
        }
    }
}
