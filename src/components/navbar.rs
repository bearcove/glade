use dioxus::prelude::*;

stylance::import_style!(style, "navbar.module.scss");

#[component]
pub fn Navbar(#[props(default = false)] sticky: bool, children: Element) -> Element {
    let sticky_class = if sticky { style::sticky } else { "" };

    rsx! {
        nav { class: stylance::classes!(style::navbar, sticky_class),
            div { class: style::container,
                {children}
            }
        }
    }
}

#[component]
pub fn NavbarBrand(children: Element) -> Element {
    rsx! {
        div { class: style::brand,
            {children}
        }
    }
}

#[component]
pub fn NavbarNav(children: Element) -> Element {
    rsx! {
        div { class: style::nav,
            {children}
        }
    }
}

#[component]
pub fn NavbarItem(
    #[props(default = false)] active: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let active_class = if active { style::item_active } else { "" };

    rsx! {
        button {
            class: stylance::classes!(style::item, active_class),
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
pub fn NavbarActions(children: Element) -> Element {
    rsx! {
        div { class: style::actions,
            {children}
        }
    }
}
