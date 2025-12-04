//! IconGrid - grid layout for showcasing icons

use dioxus::prelude::*;

stylance::import_style!(style, "icon_grid.module.scss");

/// IconGrid - displays icons in a responsive grid
#[component]
pub fn IconGrid(children: Element) -> Element {
    rsx! {
        div { class: style::grid,
            {children}
        }
    }
}

/// IconGridItem - individual icon cell with preview and name
#[component]
pub fn IconGridItem(
    /// Name to display below the icon
    name: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: style::item,
            div { class: style::preview, {children} }
            span { class: style::name, "{name}" }
        }
    }
}
