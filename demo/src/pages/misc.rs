//! Misc page - TodoList, ColorSwatch, PulsingDots, Sunburst, RetryButton, Footer, AppShell

use dioxus::prelude::*;
use glade_dioxus::{
    Button, ColorSwatch, Footer, FooterBottom, FooterCopyright, FooterLink, FooterLinks,
    FooterSection, PulsingDots, PulsingDotsSize, RetryButton, RetryButtonState, Row, RowAlign,
    Section, Stack, SubSection, Sunburst, SunburstSize, TodoItem, TodoList, TodoSize,
};

#[component]
pub fn MiscPage() -> Element {
    // Todo list state
    let mut todo1_checked = use_signal(|| false);
    let mut todo2_checked = use_signal(|| true);
    let mut todo3_checked = use_signal(|| false);

    // Retry button state
    let mut retry_state = use_signal(|| RetryButtonState::Idle);

    rsx! {
        Section { id: "todo-list".to_string(), title: "Todo List".to_string(),
            SubSection { title: "Basic".to_string(),
                TodoList {
                    TodoItem {
                        checked: todo1_checked(),
                        onchange: move |_| todo1_checked.set(!todo1_checked()),
                        "Complete the documentation"
                    }
                    TodoItem {
                        checked: todo2_checked(),
                        onchange: move |_| todo2_checked.set(!todo2_checked()),
                        "Review pull request"
                    }
                    TodoItem {
                        checked: todo3_checked(),
                        onchange: move |_| todo3_checked.set(!todo3_checked()),
                        "Deploy to production"
                    }
                }
            }
            SubSection { title: "Sizes".to_string(),
                TodoList {
                    TodoItem { size: TodoSize::Small, "Small todo item" }
                    TodoItem { size: TodoSize::Medium, "Medium todo item" }
                    TodoItem { size: TodoSize::Large, "Large todo item" }
                }
            }
            SubSection { title: "Disabled".to_string(),
                TodoList {
                    TodoItem { disabled: true, "Disabled unchecked" }
                    TodoItem { disabled: true, checked: true, "Disabled checked" }
                }
            }
        }

        Section { id: "color-swatch".to_string(), title: "Color Swatch".to_string(),
            SubSection { title: "Colors".to_string(),
                Row { align: RowAlign::Center,
                    ColorSwatch { color: "#3b82f6".to_string() }
                    ColorSwatch { color: "#22c55e".to_string() }
                    ColorSwatch { color: "#ef4444".to_string() }
                    ColorSwatch { color: "#f59e0b".to_string() }
                    ColorSwatch { color: "#8b5cf6".to_string() }
                }
            }
            SubSection { title: "With Labels".to_string(),
                Row { align: RowAlign::Center,
                    ColorSwatch { color: "#3b82f6".to_string(), label: Some("Primary".to_string()) }
                    ColorSwatch { color: "#22c55e".to_string(), label: Some("Success".to_string()) }
                    ColorSwatch { color: "#ef4444".to_string(), label: Some("Error".to_string()) }
                }
            }
            SubSection { title: "Non-copyable".to_string(),
                ColorSwatch { color: "rgb(59, 130, 246)".to_string(), copyable: false }
            }
        }

        Section { id: "pulsing-dots".to_string(), title: "Pulsing Dots".to_string(),
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Stack {
                        span { "Small" }
                        PulsingDots { size: PulsingDotsSize::Small }
                    }
                    Stack {
                        span { "Medium" }
                        PulsingDots { size: PulsingDotsSize::Medium }
                    }
                    Stack {
                        span { "Large" }
                        PulsingDots { size: PulsingDotsSize::Large }
                    }
                }
            }
            SubSection { title: "Inline Usage".to_string(),
                p {
                    "Loading"
                    PulsingDots {}
                }
            }
        }

        Section { id: "sunburst".to_string(), title: "Sunburst".to_string(),
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Stack {
                        span { "Small" }
                        Sunburst { size: SunburstSize::Small }
                    }
                    Stack {
                        span { "Medium" }
                        Sunburst { size: SunburstSize::Medium }
                    }
                    Stack {
                        span { "Large" }
                        Sunburst { size: SunburstSize::Large }
                    }
                }
            }
        }

        Section { id: "retry-button".to_string(), title: "Retry Button".to_string(),
            SubSection { title: "States".to_string(),
                Row { align: RowAlign::Center,
                    RetryButton {
                        state: RetryButtonState::Idle,
                        on_click: move |_| {},
                        idle_text: "Submit".to_string(),
                    }
                    RetryButton {
                        state: RetryButtonState::Loading,
                        on_click: move |_| {},
                    }
                    RetryButton {
                        state: RetryButtonState::Failed,
                        on_click: move |_| {},
                    }
                    RetryButton {
                        state: RetryButtonState::Success,
                        on_click: move |_| {},
                    }
                }
            }
            SubSection { title: "Interactive".to_string(),
                RetryButton {
                    state: retry_state(),
                    on_click: {
                        let mut state = retry_state;
                        move |_| {
                            let current = state();
                            let next = match current {
                                RetryButtonState::Idle => RetryButtonState::Loading,
                                RetryButtonState::Loading => RetryButtonState::Failed,
                                RetryButtonState::Failed => RetryButtonState::Loading,
                                RetryButtonState::Success => RetryButtonState::Idle,
                            };
                            state.set(next);
                        }
                    },
                }
                Button {
                    onclick: move |_| retry_state.set(RetryButtonState::Success),
                    "Set Success"
                }
            }
        }

        Section { id: "footer".to_string(), title: "Footer".to_string(),
            div { style: "border: 1px solid var(--color-border); border-radius: 0.5rem; overflow: hidden;",
                Footer {
                    Row {
                        FooterSection { title: Some("Product".to_string()),
                            FooterLinks {
                                FooterLink { href: "#".to_string(), "Features" }
                                FooterLink { href: "#".to_string(), "Pricing" }
                                FooterLink { href: "#".to_string(), "Documentation" }
                            }
                        }
                        FooterSection { title: Some("Company".to_string()),
                            FooterLinks {
                                FooterLink { href: "#".to_string(), "About" }
                                FooterLink { href: "#".to_string(), "Blog" }
                                FooterLink { href: "#".to_string(), "Careers" }
                            }
                        }
                        FooterSection { title: Some("Legal".to_string()),
                            FooterLinks {
                                FooterLink { href: "#".to_string(), "Privacy" }
                                FooterLink { href: "#".to_string(), "Terms" }
                            }
                        }
                    }
                    FooterBottom {
                        FooterCopyright { year: "2024".to_string(), holder: "Glade".to_string() }
                    }
                }
            }
        }
    }
}
