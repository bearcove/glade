//! Navigation page - Tabs, Dropdown, Modal, Navbar, Sidebar, AlertDialog, Drawer, Accordion, Collapsible, Pagination, Steps

use dioxus::prelude::*;
use glade::{
    Accordion, AccordionItem, AlertDialog, AlertDialogVariant, Button, ButtonVariant, Collapsible,
    ContextMenu, ContextMenuContent, ContextMenuDivider, ContextMenuItem, ContextMenuTrigger,
    Drawer, DrawerBody, DrawerFooter, DrawerHeader, DrawerPosition, Dropdown, DropdownDivider,
    DropdownItem, DropdownMenu, DropdownTrigger, Modal, ModalBody, ModalFooter, ModalHeader,
    Pagination, Row, Section, SimplePagination, Stack, Step, StepStatus, Steps, SubSection, Tab,
    TabList, TabPanel, TabPanels, Tabs, TabsVariant,
};

#[component]
pub fn NavigationPage() -> Element {
    let mut active_tab = use_signal(|| 0usize);
    let mut modal_open = use_signal(|| false);
    let mut alert_dialog_open = use_signal(|| false);
    let mut drawer_open = use_signal(|| false);
    let mut collapsible_open = use_signal(|| false);
    let mut current_page = use_signal(|| 1usize);
    let mut current_step = use_signal(|| 1usize);

    rsx! {
        Section { id: "tabs".to_string(), title: "Tabs".to_string(),
            SubSection { title: "Line (default)".to_string(),
                Tabs {
                    TabList {
                        Tab { active: active_tab() == 0, onclick: move |_| active_tab.set(0), "Tab 1" }
                        Tab { active: active_tab() == 1, onclick: move |_| active_tab.set(1), "Tab 2" }
                        Tab { active: active_tab() == 2, onclick: move |_| active_tab.set(2), "Tab 3" }
                    }
                    TabPanels {
                        TabPanel { active: active_tab() == 0, "Content for Tab 1" }
                        TabPanel { active: active_tab() == 1, "Content for Tab 2" }
                        TabPanel { active: active_tab() == 2, "Content for Tab 3" }
                    }
                }
            }
            SubSection { title: "Pills".to_string(),
                Tabs { variant: TabsVariant::Pills,
                    TabList {
                        Tab { active: true, "Active" }
                        Tab { "Inactive" }
                        Tab { disabled: true, "Disabled" }
                    }
                }
            }
        }

        Section { id: "dropdown".to_string(), title: "Dropdown".to_string(),
            Dropdown {
                DropdownTrigger {
                    Button { "Open Menu" }
                }
                DropdownMenu {
                    DropdownItem { "Profile" }
                    DropdownItem { "Settings" }
                    DropdownDivider {}
                    DropdownItem { "Logout" }
                }
            }
        }

        Section { id: "context-menu".to_string(), title: "Context Menu".to_string(),
            ContextMenu {
                ContextMenuTrigger {
                    div {
                        style: "padding: 2rem; border: 1px dashed var(--color-border); border-radius: 0.5rem; text-align: center; color: var(--color-text-muted);",
                        "Right-click here"
                    }
                }
                ContextMenuContent {
                    ContextMenuItem { "Cut" }
                    ContextMenuItem { "Copy" }
                    ContextMenuItem { "Paste" }
                    ContextMenuDivider {}
                    ContextMenuItem { "Select All" }
                    ContextMenuDivider {}
                    ContextMenuItem { disabled: true, "Disabled Item" }
                }
            }
        }

        Section { id: "modal".to_string(), title: "Modal".to_string(),
            Button { onclick: move |_| modal_open.set(true), "Open Modal" }
            Modal { open: modal_open(), onclose: move |_| modal_open.set(false),
                ModalHeader { onclose: move |_| modal_open.set(false), "Modal Title" }
                ModalBody {
                    p { "This is the modal content. You can put any content here." }
                }
                ModalFooter {
                    Button { variant: ButtonVariant::Secondary, onclick: move |_| modal_open.set(false), "Cancel" }
                    Button { onclick: move |_| modal_open.set(false), "Confirm" }
                }
            }
        }

        Section { id: "alert-dialog".to_string(), title: "Alert Dialog".to_string(),
            SubSection { title: "Danger confirmation".to_string(),
                Button { variant: ButtonVariant::Danger, onclick: move |_| alert_dialog_open.set(true), "Delete Item" }
                AlertDialog {
                    open: alert_dialog_open(),
                    variant: AlertDialogVariant::Danger,
                    title: "Delete this item?".to_string(),
                    description: "This action cannot be undone. The item will be permanently deleted.".to_string(),
                    confirm_text: "Delete".to_string(),
                    cancel_text: "Cancel".to_string(),
                    onconfirm: move |_| alert_dialog_open.set(false),
                    onclose: move |_| alert_dialog_open.set(false),
                }
            }
        }

        Section { id: "drawer".to_string(), title: "Drawer".to_string(),
            SubSection { title: "Right drawer".to_string(),
                Button { onclick: move |_| drawer_open.set(true), "Open Drawer" }
                Drawer { open: drawer_open, position: DrawerPosition::Right,
                    DrawerHeader { "Drawer Title" }
                    DrawerBody {
                        p { "This is drawer content. It slides in from the side." }
                    }
                    DrawerFooter {
                        Button { variant: ButtonVariant::Secondary, onclick: move |_| drawer_open.set(false), "Close" }
                        Button { onclick: move |_| drawer_open.set(false), "Save" }
                    }
                }
            }
        }

        Section { id: "accordion".to_string(), title: "Accordion".to_string(),
            Accordion {
                AccordionItem { title: "Section 1".to_string(), default_open: true,
                    p { "Content for the first section. Click the header to collapse." }
                }
                AccordionItem { title: "Section 2".to_string(),
                    p { "Content for the second section." }
                }
                AccordionItem { title: "Section 3".to_string(),
                    p { "Content for the third section." }
                }
            }
        }

        Section { id: "collapsible".to_string(), title: "Collapsible".to_string(),
            Collapsible {
                open: collapsible_open(),
                on_toggle: move |open| collapsible_open.set(open),
                trigger: rsx! { Button { "Toggle content" } },
                p { "This is collapsible content. It can be shown or hidden." }
                p { "You can put any content here." }
            }
        }

        Section { id: "pagination".to_string(), title: "Pagination".to_string(),
            SubSection { title: "Full pagination".to_string(),
                Stack {
                    Pagination {
                        current: current_page(),
                        total: 10,
                        on_change: move |page| current_page.set(page),
                    }
                    p { "Page " {current_page().to_string()} " of 10" }
                }
            }
            SubSection { title: "Simple pagination".to_string(),
                SimplePagination {
                    current: 3,
                    total: 10,
                    on_change: move |_| {},
                }
            }
        }

        Section { id: "steps".to_string(), title: "Steps".to_string(),
            SubSection { title: "Horizontal".to_string(),
                Stack {
                    Steps { current: current_step(),
                        Step { index: 0, title: "Account".to_string(), description: "Create your account".to_string() }
                        Step { index: 1, title: "Profile".to_string(), description: "Set up your profile".to_string() }
                        Step { index: 2, title: "Finish".to_string(), description: "Complete setup".to_string() }
                    }
                    Row {
                        Button {
                            disabled: current_step() == 0,
                            onclick: move |_| current_step.set(current_step().saturating_sub(1)),
                            "Previous"
                        }
                        Button {
                            disabled: current_step() >= 2,
                            onclick: move |_| current_step.set(current_step() + 1),
                            "Next"
                        }
                    }
                }
            }
            SubSection { title: "With error".to_string(),
                Steps { current: 1,
                    Step { index: 0, title: "Step 1".to_string(), status: StepStatus::Completed }
                    Step { index: 1, title: "Step 2".to_string(), status: StepStatus::Error }
                    Step { index: 2, title: "Step 3".to_string(), status: StepStatus::Pending }
                }
            }
        }
    }
}
