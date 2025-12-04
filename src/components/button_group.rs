//! ButtonGroup - segmented button container

use dioxus::prelude::*;

stylance::import_style!(style, "button_group.module.scss");

/// ButtonGroup - groups buttons together with no gap and shared border radius
///
/// The first button gets left border radius, the last gets right border radius,
/// middle buttons have no border radius. Borders are collapsed to avoid doubling.
#[component]
pub fn ButtonGroup(children: Element) -> Element {
    rsx! {
        div { class: style::button_group,
            {children}
        }
    }
}
