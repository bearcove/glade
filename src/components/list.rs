//! List component for styled lists with items

use dioxus::prelude::*;

stylance::import_style!(style, "list.module.scss");

/// Size variants for list
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ListSize {
    /// Small list items
    Small,
    /// Medium list items (default)
    #[default]
    Medium,
    /// Large list items
    Large,
}

/// Visual variants for list
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ListVariant {
    /// Simple list without borders
    #[default]
    Plain,
    /// List with borders between items
    Bordered,
    /// List with background on items
    Filled,
}

/// A styled list container
#[component]
pub fn List(
    /// List items
    children: Element,
    /// Size variant
    #[props(default)]
    size: ListSize,
    /// Visual variant
    #[props(default)]
    variant: ListVariant,
    /// Whether the list is ordered (numbered)
    #[props(default = false)]
    ordered: bool,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let size_class = match size {
        ListSize::Small => style::small,
        ListSize::Medium => "",
        ListSize::Large => style::large,
    };

    let variant_class = match variant {
        ListVariant::Plain => "",
        ListVariant::Bordered => style::bordered,
        ListVariant::Filled => style::filled,
    };

    let classes = stylance::classes!(
        style::list,
        size_class,
        variant_class,
        class.as_deref().unwrap_or("")
    );

    if ordered {
        rsx! {
            ol { class: classes,
                {children}
            }
        }
    } else {
        rsx! {
            ul { class: classes,
                {children}
            }
        }
    }
}

/// An individual list item
#[component]
pub fn ListItem(
    /// Item content
    children: Element,
    /// Optional leading icon/element
    #[props(optional)]
    leading: Option<Element>,
    /// Optional trailing element (action, badge, etc.)
    #[props(optional)]
    trailing: Option<Element>,
    /// Whether the item is clickable
    #[props(default = false)]
    clickable: bool,
    /// Click handler
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Whether the item is selected/active
    #[props(default = false)]
    selected: bool,
    /// Whether the item is disabled
    #[props(default = false)]
    disabled: bool,
) -> Element {
    rsx! {
        li {
            class: stylance::classes!(
                style::list_item,
                if clickable { style::clickable } else { "" },
                if selected { style::selected } else { "" },
                if disabled { style::disabled } else { "" }
            ),
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &onclick {
                        handler.call(evt);
                    }
                }
            },

            if let Some(lead) = leading {
                span { class: style::leading,
                    {lead}
                }
            }

            span { class: style::content,
                {children}
            }

            if let Some(trail) = trailing {
                span { class: style::trailing,
                    {trail}
                }
            }
        }
    }
}

/// A list item with title and description
#[component]
pub fn ListItemContent(
    /// Main title
    #[props(into)]
    title: String,
    /// Optional description
    #[props(optional, into)]
    description: Option<String>,
) -> Element {
    rsx! {
        div { class: style::item_content,
            span { class: style::item_title, "{title}" }
            if let Some(desc) = description {
                span { class: style::item_description, "{desc}" }
            }
        }
    }
}

/// A section header for grouped lists
#[component]
pub fn ListSection(
    /// Section title
    #[props(into)]
    title: String,
    /// Section items
    children: Element,
) -> Element {
    rsx! {
        li { class: style::list_section,
            span { class: style::section_title, "{title}" }
            ul { class: style::section_items,
                {children}
            }
        }
    }
}
