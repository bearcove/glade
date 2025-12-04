use dioxus::prelude::*;

stylance::import_style!(style, "empty_state.module.scss");

#[component]
pub fn EmptyState(
    #[props(default)] title: String,
    #[props(default)] description: String,
    #[props(default)] icon: Option<Element>,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        div { class: style::empty_state,
            if let Some(icon_el) = icon {
                div { class: style::icon,
                    {icon_el}
                }
            }
            if !title.is_empty() {
                h3 { class: style::title, "{title}" }
            }
            if !description.is_empty() {
                p { class: style::description, "{description}" }
            }
            if let Some(action_el) = action {
                div { class: style::action,
                    {action_el}
                }
            }
        }
    }
}
