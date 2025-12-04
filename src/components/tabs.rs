//! Tabs component for organizing content into switchable panels

use dioxus::prelude::*;

stylance::import_style!(style, "tabs.module.scss");

/// Visual style variants for Tabs
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    /// Line tabs with underline indicator (default)
    #[default]
    Line,
    /// Enclosed tabs with border
    Enclosed,
    /// Pill-shaped tabs with background
    Pills,
}

#[component]
pub fn Tabs(#[props(default)] variant: TabsVariant, children: Element) -> Element {
    let variant_class = match variant {
        TabsVariant::Line => "",
        TabsVariant::Enclosed => style::enclosed,
        TabsVariant::Pills => style::pills,
    };

    rsx! {
        div { class: stylance::classes!(style::tabs, variant_class),
            {children}
        }
    }
}

#[component]
pub fn TabList(children: Element) -> Element {
    rsx! {
        div { class: style::list, role: "tablist",
            {children}
        }
    }
}

#[component]
pub fn Tab(
    #[props(default = false)] active: bool,
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let active_class = if active { style::tab_active } else { "" };

    rsx! {
        button {
            class: stylance::classes!(style::tab, active_class),
            role: "tab",
            "aria-selected": active,
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
pub fn TabPanel(#[props(default = false)] active: bool, children: Element) -> Element {
    let active_class = if active { style::panel_active } else { "" };

    rsx! {
        div {
            class: stylance::classes!(style::panel, active_class),
            role: "tabpanel",
            hidden: !active,
            {children}
        }
    }
}

#[component]
pub fn TabPanels(children: Element) -> Element {
    rsx! {
        div { class: style::panels,
            {children}
        }
    }
}
