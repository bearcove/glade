//! Card component for grouping related content

use dioxus::prelude::*;

stylance::import_style!(#[allow(dead_code)] style, "card.module.scss");

#[component]
pub fn Card(
    #[props(default = false)] hoverable: bool,
    #[props(default = false)] ghost: bool,
    children: Element,
) -> Element {
    let variant_class = if ghost {
        style::ghost
    } else if hoverable {
        style::hoverable
    } else {
        ""
    };

    rsx! {
        div { class: stylance::classes!(style::card, variant_class),
            {children}
        }
    }
}

#[component]
pub fn CardHeader(children: Element) -> Element {
    rsx! {
        div { class: style::header,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(children: Element) -> Element {
    rsx! {
        h3 { class: style::title,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(children: Element) -> Element {
    rsx! {
        p { class: style::description,
            {children}
        }
    }
}

#[component]
pub fn CardContent(children: Element) -> Element {
    rsx! {
        div { class: style::content,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(children: Element) -> Element {
    rsx! {
        div { class: style::footer,
            {children}
        }
    }
}
