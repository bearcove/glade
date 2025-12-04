//! `AppShell` layout component for standard app structure

use dioxus::prelude::*;

stylance::import_style!(style, "app_shell.module.scss");

/// Main application shell with header, sidebar, and content
#[component]
pub fn AppShell(
    /// Header content
    header: Option<Element>,
    /// Sidebar content
    sidebar: Option<Element>,
    /// Footer content
    footer: Option<Element>,
    /// Sidebar width (CSS value)
    #[props(default = "240px".to_string())]
    sidebar_width: String,
    /// Header height (CSS value)
    #[props(default = "56px".to_string())]
    header_height: String,
    /// Whether sidebar is collapsible (reserved for future use)
    #[props(default = false)]
    _collapsible: bool,
    /// Whether sidebar is currently collapsed
    #[props(default = false)]
    collapsed: bool,
    /// Main content
    children: Element,
) -> Element {
    let sidebar_style = if collapsed {
        "width: 0; overflow: hidden;".to_string()
    } else {
        format!("width: {sidebar_width};")
    };

    let main_style = if sidebar.is_some() && !collapsed {
        format!("margin-left: {sidebar_width};")
    } else {
        String::new()
    };

    let header_style = format!("height: {header_height};");

    rsx! {
        div { class: style::app_shell,
            if let Some(h) = header {
                header { class: style::header, style: "{header_style}", {h} }
            }
            div { class: style::body,
                if let Some(s) = sidebar {
                    aside { class: style::sidebar, style: "{sidebar_style}", {s} }
                }
                main { class: style::main, style: "{main_style}", {children} }
            }
            if let Some(f) = footer {
                footer { class: style::footer, {f} }
            }
        }
    }
}

/// Simple header bar for AppShell
#[component]
pub fn AppHeader(
    /// Left section (brand, logo)
    left: Option<Element>,
    /// Center section (navigation, search)
    center: Option<Element>,
    /// Right section (actions, user menu)
    right: Option<Element>,
) -> Element {
    rsx! {
        div { class: style::app_header,
            div { class: style::app_header_left, {left} }
            div { class: style::app_header_center, {center} }
            div { class: style::app_header_right, {right} }
        }
    }
}

/// Content area with optional padding
#[component]
pub fn AppContent(
    /// Add default padding
    #[props(default = true)]
    padded: bool,
    /// Maximum width (CSS value, empty for full width)
    max_width: Option<String>,
    /// Center the content
    #[props(default = false)]
    centered: bool,
    children: Element,
) -> Element {
    let mut styles = Vec::new();
    if let Some(mw) = max_width {
        styles.push(format!("max-width: {mw}"));
    }
    if centered {
        styles.push("margin-left: auto; margin-right: auto".to_string());
    }
    let style_str = if styles.is_empty() {
        None
    } else {
        Some(styles.join("; "))
    };

    rsx! {
        div {
            class: stylance::classes!(style::app_content, if padded { style::padded } else { "" }),
            style: style_str,
            {children}
        }
    }
}
