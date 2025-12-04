//! Command palette component with keyboard navigation and global shortcut support

use std::rc::Rc;

use dioxus::events::MountedData;
use dioxus::prelude::*;

use crate::components::icons::{IconSearch, IconX};
#[cfg(target_arch = "wasm32")]
use crate::hooks::use_callback_channel;

stylance::import_style!(style, "command_palette.module.scss");

/// A single item in the command palette
#[derive(Clone, PartialEq)]
pub struct CommandItem {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional group/category
    pub group: Option<String>,
}

/// Command palette with search, keyboard navigation, and optional Cmd+K shortcut
#[component]
pub fn CommandPalette(
    /// Placeholder text for the search input
    #[props(default = "Search...".to_string())]
    placeholder: String,
    /// The list of items to search through
    items: Signal<Vec<CommandItem>>,
    /// Callback when an item is selected
    onselect: EventHandler<CommandItem>,
    /// Whether to show the Cmd+K hint in the input
    #[props(default = true)]
    show_shortcut_hint: bool,
    /// Whether to register global Cmd+K / Ctrl+K shortcut
    #[props(default = true)]
    global_shortcut: bool,
) -> Element {
    let mut query = use_signal(String::new);
    let mut is_open = use_signal(|| false);
    let mut selected_index = use_signal(|| 0_i32);
    // Store the mounted input element for focusing via onmounted
    // REMINDER: NEVER use get_element_by_id for focus - use onmounted + set_focus()
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    // Filter items based on query
    let filtered_items = use_memo(move || {
        let q = query().to_lowercase();
        if q.len() < 2 {
            return vec![];
        }
        items()
            .into_iter()
            .filter(|item| {
                item.name.to_lowercase().contains(&q)
                    || item
                        .description
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&q))
                    || item.id.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    });

    // Reset selection when query changes
    use_effect(move || {
        let _ = query();
        selected_index.set(0);
    });

    // Global Cmd+K / Ctrl+K shortcut
    // REMINDER: Use onmounted + set_focus(), NEVER get_element_by_id
    // Uses use_callback_channel to bridge sync EventListener to async Dioxus runtime
    #[cfg(target_arch = "wasm32")]
    let focus_sender = use_callback_channel(move |()| async move {
        if let Some(el) = input_element.cloned() {
            let _ = el.set_focus(true).await;
        }
    });

    #[cfg(target_arch = "wasm32")]
    let _cmd_k_listener = {
        use_signal(move || {
            use gloo_events::{EventListener, EventListenerOptions};
            use wasm_bindgen::JsCast;

            if !global_shortcut {
                return None;
            }

            let window = web_sys::window()?;
            let options = EventListenerOptions::enable_prevent_default();
            let sender = focus_sender.clone();

            Some(EventListener::new_with_options(
                &window,
                "keydown",
                options,
                move |event| {
                    let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
                        return;
                    };
                    if (event.meta_key() || event.ctrl_key()) && event.key() == "k" {
                        event.prevent_default();
                        is_open.set(true);
                        sender.send(());
                    }
                },
            ))
        })
    };

    let has_results = filtered_items().len() > 0 && is_open();

    // Handle keyboard navigation
    let handle_keydown = move |ev: KeyboardEvent| {
        let results = filtered_items();
        let len = results.len() as i32;

        match ev.key() {
            Key::ArrowDown => {
                ev.prevent_default();
                if len > 0 {
                    selected_index.set((selected_index() + 1).min(len - 1));
                }
            }
            Key::ArrowUp => {
                ev.prevent_default();
                if len > 0 {
                    selected_index.set((selected_index() - 1).max(0));
                }
            }
            Key::Enter => {
                ev.prevent_default();
                let idx = selected_index() as usize;
                if let Some(item) = results.get(idx) {
                    onselect.call(item.clone());
                    query.set(String::new());
                    is_open.set(false);
                }
            }
            Key::Escape => {
                is_open.set(false);
                query.set(String::new());
            }
            _ => {}
        }
    };

    let wrapper_class = if is_open() {
        stylance::classes!(style::input_wrapper, style::open)
    } else {
        style::input_wrapper.to_string()
    };

    rsx! {
        div { class: style::container,
            div { class: "{wrapper_class}",
                span { class: style::search_icon, IconSearch {} }
                input {
                    r#type: "text",
                    class: style::input,
                    placeholder: "{placeholder}",
                    value: "{query}",
                    // Capture element ref for focusing via Cmd+K
                    onmounted: move |ev| input_element.set(Some(ev.data())),
                    oninput: move |ev| {
                        query.set(ev.value());
                        is_open.set(true);
                    },
                    onfocus: move |_| is_open.set(true),
                    onblur: move |_| {
                        // Delay closing to allow click events on results
                        spawn(async move {
                            #[cfg(target_arch = "wasm32")]
                            {
                                gloo_timers::future::TimeoutFuture::new(200).await;
                            }
                            is_open.set(false);
                        });
                    },
                    onkeydown: handle_keydown,
                }
                if show_shortcut_hint && query().is_empty() {
                    div { class: style::shortcut_hint,
                        kbd { class: style::kbd, "⌘" }
                        kbd { class: style::kbd, "K" }
                    }
                } else if !query().is_empty() {
                    button {
                        class: style::clear_button,
                        onclick: move |_| {
                            query.set(String::new());
                        },
                        IconX {}
                    }
                }
            }

            if has_results {
                div { class: style::results,
                    for (idx, item) in filtered_items().into_iter().enumerate() {
                        {
                            let item_clone = item.clone();
                            let is_selected = selected_index() == idx as i32;
                            let item_class = if is_selected {
                                stylance::classes!(style::result_item, style::selected)
                            } else {
                                style::result_item.to_string()
                            };
                            rsx! {
                                button {
                                    key: "{item.id}",
                                    class: "{item_class}",
                                    onmouseenter: move |_| selected_index.set(idx as i32),
                                    onclick: move |_| {
                                        onselect.call(item_clone.clone());
                                        query.set(String::new());
                                        is_open.set(false);
                                    },
                                    span { class: style::result_name, "{item.name}" }
                                    if let Some(desc) = &item.description {
                                        span { class: style::result_desc, "{desc}" }
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
