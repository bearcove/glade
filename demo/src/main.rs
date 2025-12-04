//! Glade Dioxus Demo - showcasing all components
//!
//! IMPORTANT: This demo should use ONLY glade-dioxus components.
//! No custom CSS classes - if something needs styling, add a component to glade-dioxus.
//! The demo.css file should ONLY contain CSS variables for theming.

mod pages;

use dioxus::prelude::*;
use dioxus::router::{use_navigator, use_route, Link, Routable, Router};
use glade::{
    Breadcrumb, BreadcrumbItem, Button, ButtonVariant, CommandItem, CommandPalette, Container,
    IconAlertCircle, IconCheck, IconChevronRight, IconFileText, IconLoader, IconMenu, IconPlus,
    IconSearch, MainContent, Navbar, NavbarActions, NavbarBrand, PageNav, Sidebar, SidebarItem,
    SidebarLayout, SidebarNav, GLADE_BASE_CSS, GLADE_STYLANCE_CSS,
};
use pages::{ChatPage, DataPage, FeedbackPage, FormPage, LayoutPage, LoadingPage, MiscPage, NavigationPage, PrimitivesPage};

const DEMO_CSS: Asset = asset!("/css/demo.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

/// Component registry entry
#[derive(Clone, Debug)]
pub struct ComponentEntry {
    /// Component ID (used in URL hash)
    pub id: &'static str,
    /// Display name
    pub name: &'static str,
    /// Short description
    pub description: &'static str,
    /// Group/section this component belongs to
    pub group: &'static str,
}

/// All available groups
const GROUPS: &[(&str, &str)] = &[
    ("primitives", "Primitives"),
    ("form", "Form"),
    ("loading", "Loading"),
    ("feedback", "Feedback"),
    ("navigation", "Navigation"),
    ("layout", "Layout"),
    ("chat", "Chat"),
    ("data", "Data"),
    ("misc", "Misc"),
];

/// Component registry - all components organized by group
const COMPONENTS: &[ComponentEntry] = &[
    // Primitives
    ComponentEntry { id: "icons", name: "Icons", description: "SVG icons for common actions", group: "primitives" },
    ComponentEntry { id: "button", name: "Button", description: "Clickable button with variants", group: "primitives" },
    ComponentEntry { id: "button-group", name: "Button Group", description: "Group of related buttons", group: "primitives" },
    ComponentEntry { id: "avatar", name: "Avatar", description: "User profile image or initials", group: "primitives" },
    ComponentEntry { id: "badge", name: "Badge", description: "Status indicator labels", group: "primitives" },
    ComponentEntry { id: "card", name: "Card", description: "Content container with sections", group: "primitives" },
    // Form
    ComponentEntry { id: "input", name: "Input", description: "Text input with validation", group: "form" },
    ComponentEntry { id: "textarea", name: "Textarea", description: "Multi-line text input", group: "form" },
    ComponentEntry { id: "select", name: "Select", description: "Dropdown selection input", group: "form" },
    ComponentEntry { id: "checkbox", name: "Checkbox", description: "Binary choice input", group: "form" },
    ComponentEntry { id: "radio", name: "Radio", description: "Single choice from options", group: "form" },
    ComponentEntry { id: "toggle", name: "Toggle", description: "Toggle for on/off states", group: "form" },
    ComponentEntry { id: "slider", name: "Slider", description: "Range input for values", group: "form" },
    ComponentEntry { id: "file-input", name: "File Input", description: "File upload with drag-and-drop", group: "form" },
    ComponentEntry { id: "rating", name: "Rating", description: "Star rating input", group: "form" },
    ComponentEntry { id: "label", name: "Label", description: "Form field label with accessibility", group: "form" },
    ComponentEntry { id: "segmented-input", name: "Segmented Input", description: "OTP/code input with segments", group: "form" },
    ComponentEntry { id: "calendar", name: "Calendar", description: "Date picker calendar", group: "form" },
    // Loading
    ComponentEntry { id: "progress", name: "Progress", description: "Progress indicators", group: "loading" },
    ComponentEntry { id: "spinner", name: "Spinner", description: "Circular loading indicator", group: "loading" },
    // Feedback
    ComponentEntry { id: "alert", name: "Alert", description: "Informational message banners", group: "feedback" },
    ComponentEntry { id: "tooltip", name: "Tooltip", description: "Hover information popup", group: "feedback" },
    ComponentEntry { id: "empty-state", name: "Empty State", description: "Placeholder for empty content", group: "feedback" },
    ComponentEntry { id: "toast", name: "Toast", description: "Toast notifications", group: "feedback" },
    // Navigation
    ComponentEntry { id: "tabs", name: "Tabs", description: "Tabbed content navigation", group: "navigation" },
    ComponentEntry { id: "dropdown", name: "Dropdown", description: "Dropdown menu with actions", group: "navigation" },
    ComponentEntry { id: "context-menu", name: "Context Menu", description: "Right-click menu", group: "navigation" },
    ComponentEntry { id: "modal", name: "Modal", description: "Dialog overlay for content", group: "navigation" },
    ComponentEntry { id: "alert-dialog", name: "Alert Dialog", description: "Confirmation dialogs", group: "navigation" },
    ComponentEntry { id: "drawer", name: "Drawer", description: "Slide-over panel", group: "navigation" },
    ComponentEntry { id: "accordion", name: "Accordion", description: "Collapsible content sections", group: "navigation" },
    ComponentEntry { id: "collapsible", name: "Collapsible", description: "Expandable content panel", group: "navigation" },
    ComponentEntry { id: "pagination", name: "Pagination", description: "Page navigation controls", group: "navigation" },
    ComponentEntry { id: "steps", name: "Steps", description: "Multi-step progress indicator", group: "navigation" },
    // Layout
    ComponentEntry { id: "separator", name: "Separator", description: "Visual content divider", group: "layout" },
    ComponentEntry { id: "aspect-ratio", name: "Aspect Ratio", description: "Maintain aspect ratio container", group: "layout" },
    ComponentEntry { id: "scroll-area", name: "Scroll Area", description: "Custom scrollbar container", group: "layout" },
    ComponentEntry { id: "carousel", name: "Carousel", description: "Image/content slider", group: "layout" },
    ComponentEntry { id: "split-pane", name: "Split Pane", description: "Resizable panels", group: "layout" },
    // Chat
    ComponentEntry { id: "chat-bubble", name: "Chat Bubble", description: "Message bubble with variants", group: "chat" },
    ComponentEntry { id: "message-group", name: "Message Group", description: "Grouped messages from same author", group: "chat" },
    ComponentEntry { id: "message-list", name: "Message List", description: "Scrollable message container", group: "chat" },
    ComponentEntry { id: "message-composer", name: "Message Composer", description: "Chat input with send button", group: "chat" },
    ComponentEntry { id: "thread-list", name: "Thread List", description: "Conversation list navigation", group: "chat" },
    ComponentEntry { id: "day-divider", name: "Day Divider", description: "Date separators in messages", group: "chat" },
    ComponentEntry { id: "streaming-status", name: "Streaming Status", description: "Activity indicators", group: "chat" },
    ComponentEntry { id: "attachment-chip", name: "Attachment Chip", description: "File attachment display", group: "chat" },
    ComponentEntry { id: "notification-badge", name: "Notification Badge", description: "Unread count badges", group: "chat" },
    // Data
    ComponentEntry { id: "diff-stats", name: "Diff Stats", description: "Git diff statistics display", group: "data" },
    ComponentEntry { id: "tool-call-badge", name: "Tool Call Badge", description: "Tool execution status", group: "data" },
    ComponentEntry { id: "code-execution-result", name: "Code Execution Result", description: "Command output panel", group: "data" },
    ComponentEntry { id: "code-block", name: "Code Block", description: "Syntax highlighted code", group: "data" },
    ComponentEntry { id: "stat", name: "Stat", description: "Statistics/metrics display", group: "data" },
    ComponentEntry { id: "list", name: "List", description: "Styled list with items", group: "data" },
    ComponentEntry { id: "descriptions", name: "Descriptions", description: "Key-value pairs display", group: "data" },
    ComponentEntry { id: "table", name: "Table", description: "Tabular data display", group: "data" },
    ComponentEntry { id: "hover-card", name: "Hover Card", description: "Rich content on hover", group: "data" },
    ComponentEntry { id: "popover", name: "Popover", description: "Click-triggered popup", group: "data" },
    // Misc
    ComponentEntry { id: "todo-list", name: "Todo List", description: "Checkable task list", group: "misc" },
    ComponentEntry { id: "color-swatch", name: "Color Swatch", description: "Inline color display", group: "misc" },
    ComponentEntry { id: "pulsing-dots", name: "Pulsing Dots", description: "Animated loading dots", group: "misc" },
    ComponentEntry { id: "sunburst", name: "Sunburst", description: "Rotating loading indicator", group: "misc" },
    ComponentEntry { id: "retry-button", name: "Retry Button", description: "Button with retry states", group: "misc" },
    ComponentEntry { id: "footer", name: "Footer", description: "Site footer component", group: "misc" },
];

/// Get components for a specific group
fn components_for_group(group: &str) -> impl Iterator<Item = &'static ComponentEntry> {
    COMPONENTS.iter().filter(move |c| c.group == group)
}

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Primitives,
    #[route("/primitives")]
    PrimitivesAlt,
    #[route("/form")]
    Form,
    #[route("/loading")]
    Loading,
    #[route("/feedback")]
    Feedback,
    #[route("/navigation")]
    Navigation,
    #[route("/layout")]
    Layout,
    #[route("/chat")]
    Chat,
    #[route("/data")]
    Data,
    #[route("/misc")]
    Misc,
}

impl Route {
    fn group_id(&self) -> &'static str {
        match self {
            Self::Primitives | Self::PrimitivesAlt => "primitives",
            Self::Form => "form",
            Self::Loading => "loading",
            Self::Feedback => "feedback",
            Self::Navigation => "navigation",
            Self::Layout => "layout",
            Self::Chat => "chat",
            Self::Data => "data",
            Self::Misc => "misc",
        }
    }
}

fn route_for_group(group: &str) -> Route {
    match group {
        "primitives" => Route::Primitives,
        "form" => Route::Form,
        "loading" => Route::Loading,
        "feedback" => Route::Feedback,
        "navigation" => Route::Navigation,
        "layout" => Route::Layout,
        "chat" => Route::Chat,
        "data" => Route::Data,
        "misc" => Route::Misc,
        _ => Route::Primitives,
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Public+Sans:ital,wght@0,100..900;1,100..900&display=swap",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;1,100;1,200;1,300;1,400;1,500;1,600;1,700&display=swap",
        }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Link { rel: "stylesheet", href: GLADE_BASE_CSS }
        document::Link { rel: "stylesheet", href: GLADE_STYLANCE_CSS }
        document::Link { rel: "stylesheet", href: DEMO_CSS }

        Router::<Route> {}
    }
}

#[component]
fn Primitives() -> Element {
    rsx! { Demo { PrimitivesPage {} } }
}

#[component]
fn PrimitivesAlt() -> Element {
    rsx! { Demo { PrimitivesPage {} } }
}

#[component]
fn Form() -> Element {
    rsx! { Demo { FormPage {} } }
}

#[component]
fn Loading() -> Element {
    rsx! { Demo { LoadingPage {} } }
}

#[component]
fn Feedback() -> Element {
    rsx! { Demo { FeedbackPage {} } }
}

#[component]
fn Navigation() -> Element {
    rsx! { Demo { NavigationPage {} } }
}

#[component]
fn Layout() -> Element {
    rsx! { Demo { LayoutPage {} } }
}

#[component]
fn Chat() -> Element {
    rsx! { Demo { ChatPage {} } }
}

#[component]
fn Data() -> Element {
    rsx! { Demo { DataPage {} } }
}

#[component]
fn Misc() -> Element {
    rsx! { Demo { MiscPage {} } }
}

/// Layout shell component - header, sidebar, and main content area
#[component]
fn Demo(children: Element) -> Element {
    let route = use_route::<Route>();
    let current_group = route.group_id();
    let navigator = use_navigator();

    // Find current group index and calculate prev/next
    let current_idx = GROUPS.iter().position(|(id, _)| *id == current_group).unwrap_or(0);
    let prev_page = if current_idx > 0 {
        Some(GROUPS[current_idx - 1])
    } else {
        None
    };
    let next_page = if current_idx < GROUPS.len() - 1 {
        Some(GROUPS[current_idx + 1])
    } else {
        None
    };

    // Get current group title for breadcrumb
    let current_title = GROUPS
        .iter()
        .find(|(id, _)| *id == current_group)
        .map(|(_, title)| *title)
        .unwrap_or("Components");

    // Convert COMPONENTS to CommandItems for the CommandPalette
    let command_items: Signal<Vec<CommandItem>> = use_signal(|| {
        COMPONENTS
            .iter()
            .map(|c| CommandItem {
                id: format!("{}#{}", c.group, c.id),
                name: c.name.to_string(),
                description: Some(c.description.to_string()),
                group: Some(c.group.to_string()),
            })
            .collect()
    });

    // Handle selection - navigate to the component
    let on_select = move |item: CommandItem| {
        navigator.push(format!("/{}", item.id));
    };

    rsx! {
        div {
            // Top navbar
            Navbar { sticky: true,
                NavbarBrand {
                    Breadcrumb {
                        BreadcrumbItem { href: "/".to_string(),
                            "Glade"
                        }
                        BreadcrumbItem { current: true,
                            "{current_title}"
                        }
                    }
                }
                NavbarActions {
                    CommandPalette {
                        placeholder: "Search components...".to_string(),
                        items: command_items,
                        onselect: on_select,
                    }
                    Button { variant: ButtonVariant::Ghost, "GitHub" }
                    Button { variant: ButtonVariant::Primary, "Get Started" }
                }
            }

            // Content area with sidebar and main
            SidebarLayout {
                // Left sidebar - just page groups
                Sidebar {
                    SidebarNav {
                        for (group_id, group_title) in GROUPS.iter() {
                            {
                                let icon: Element = match *group_id {
                                    "primitives" => rsx! { IconMenu {} },
                                    "form" => rsx! { IconCheck {} },
                                    "loading" => rsx! { IconLoader {} },
                                    "feedback" => rsx! { IconAlertCircle {} },
                                    "navigation" => rsx! { IconChevronRight {} },
                                    "chat" => rsx! { IconSearch {} },
                                    "data" => rsx! { IconFileText {} },
                                    "misc" => rsx! { IconPlus {} },
                                    _ => rsx! { IconMenu {} },
                                };
                                let is_active = current_group == *group_id;

                                rsx! {
                                    Link { to: route_for_group(group_id),
                                        SidebarItem { active: is_active,
                                            {icon}
                                            {*group_title}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Main content with right-side component nav
                MainContent {
                    div { class: "content-with-toc",
                        // Main content area
                        div { class: "main-content-area",
                            Container {
                                {children}

                                // Bottom page navigation
                                PageNav {
                                    prev_label: prev_page.map(|(_, title)| title.to_string()).unwrap_or_default(),
                                    prev_href: prev_page.map(|(id, _)| format!("/{id}")).unwrap_or_default(),
                                    next_label: next_page.map(|(_, title)| title.to_string()).unwrap_or_default(),
                                    next_href: next_page.map(|(id, _)| format!("/{id}")).unwrap_or_default(),
                                }
                            }
                        }

                        // Right sidebar - component TOC for current page
                        nav { class: "toc-sidebar",
                            div { class: "toc-title", "On this page" }
                            {
                                let components: Vec<_> = components_for_group(current_group).collect();
                                rsx! {
                                    for comp in components {
                                        a { href: "#{comp.id}", class: "toc-item",
                                            {comp.name}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
