//! Table component for displaying tabular data

use dioxus::prelude::*;

stylance::import_style!(style, "table.module.scss");

/// Table size variants
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TableSize {
    /// Compact table with less padding
    Small,
    /// Default table size
    #[default]
    Medium,
    /// Larger table with more padding
    Large,
}

/// Table variant styles
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TableVariant {
    /// Default table style
    #[default]
    Default,
    /// Striped rows for easier reading
    Striped,
    /// Bordered cells
    Bordered,
}

/// Table container component
#[component]
pub fn Table(
    /// Size variant
    #[props(default)]
    size: TableSize,
    /// Style variant
    #[props(default)]
    variant: TableVariant,
    /// Whether rows are hoverable
    #[props(default = false)]
    hoverable: bool,
    /// Whether the table has a sticky header
    #[props(default = false)]
    sticky_header: bool,
    /// Table content (thead, tbody, etc.)
    children: Element,
) -> Element {
    let size_class = match size {
        TableSize::Small => style::small,
        TableSize::Medium => style::medium,
        TableSize::Large => style::large,
    };

    let variant_class = match variant {
        TableVariant::Default => "",
        TableVariant::Striped => style::striped,
        TableVariant::Bordered => style::bordered,
    };

    let hoverable_class = if hoverable { style::hoverable } else { "" };
    let sticky_class = if sticky_header { style::sticky_header } else { "" };

    rsx! {
        table { class: stylance::classes!(style::table, size_class, variant_class, hoverable_class, sticky_class),
            {children}
        }
    }
}

/// Table header component
#[component]
pub fn TableHeader(children: Element) -> Element {
    rsx! {
        thead { class: style::thead, {children} }
    }
}

/// Table body component
#[component]
pub fn TableBody(children: Element) -> Element {
    rsx! {
        tbody { class: style::tbody, {children} }
    }
}

/// Table footer component
#[component]
pub fn TableFooter(children: Element) -> Element {
    rsx! {
        tfoot { class: style::tfoot, {children} }
    }
}

/// Table row component
#[component]
pub fn TableRow(
    /// Whether this row is selected
    #[props(default = false)]
    selected: bool,
    /// Click handler
    onclick: Option<EventHandler<()>>,
    /// Row content (TableCell or TableHeaderCell)
    children: Element,
) -> Element {
    let clickable = onclick.is_some();
    let selected_class = if selected { style::selected } else { "" };
    let clickable_class = if clickable { style::clickable } else { "" };

    rsx! {
        tr {
            class: stylance::classes!(style::tr, selected_class, clickable_class),
            onclick: move |_| {
                if let Some(handler) = &onclick {
                    handler.call(());
                }
            },
            {children}
        }
    }
}

/// Table header cell component
#[component]
pub fn TableHeaderCell(
    /// Whether this column is sortable
    #[props(default = false)]
    sortable: bool,
    /// Current sort direction (None, Some("asc"), Some("desc"))
    sort_direction: Option<String>,
    /// Click handler for sorting
    onclick: Option<EventHandler<()>>,
    /// Text alignment
    #[props(default = "left".to_string())]
    align: String,
    /// Cell content
    children: Element,
) -> Element {
    let sortable_class = if sortable { style::sortable } else { "" };
    let sort_class = match sort_direction.as_deref() {
        Some("asc") => style::sort_asc,
        Some("desc") => style::sort_desc,
        _ => "",
    };

    let sort_icon = if sortable {
        match sort_direction.as_deref() {
            Some("asc") => "↑",
            Some("desc") => "↓",
            _ => "↕",
        }
    } else {
        ""
    };

    rsx! {
        th {
            class: stylance::classes!(style::th, sortable_class, sort_class),
            style: "text-align: {align}",
            onclick: move |_| {
                if let Some(handler) = &onclick {
                    handler.call(());
                }
            },
            div { class: style::th_content,
                {children}
                if sortable {
                    span { class: style::sort_icon, "{sort_icon}" }
                }
            }
        }
    }
}

/// Table data cell component
#[component]
pub fn TableCell(
    /// Text alignment
    #[props(default = "left".to_string())]
    align: String,
    /// Column span
    colspan: Option<u32>,
    /// Row span
    rowspan: Option<u32>,
    /// Cell content
    children: Element,
) -> Element {
    rsx! {
        td {
            class: style::td,
            style: "text-align: {align}",
            colspan: colspan.map(|c| c.to_string()),
            rowspan: rowspan.map(|r| r.to_string()),
            {children}
        }
    }
}

/// Empty table state component
#[component]
pub fn TableEmpty(
    /// Number of columns to span
    colspan: u32,
    /// Empty message
    #[props(default = "No data available".to_string())]
    message: String,
) -> Element {
    rsx! {
        tr {
            td { class: style::empty, colspan: "{colspan}", "{message}" }
        }
    }
}
