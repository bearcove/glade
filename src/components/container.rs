//! Container - centered content container with max-width

use dioxus::prelude::*;

stylance::import_style!(style, "container.module.scss");

/// Max width for Container
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ContainerSize {
    /// Small container (640px)
    Small,
    /// Medium container (800px)
    #[default]
    Medium,
    /// Large container (1024px)
    Large,
    /// Extra large container (1280px)
    XLarge,
}

/// Container - centers content with a max-width
#[component]
pub fn Container(
    #[props(default)] size: ContainerSize,
    #[props(default = true)] padding: bool,
    children: Element,
) -> Element {
    let size_class = match size {
        ContainerSize::Small => style::small,
        ContainerSize::Medium => style::medium,
        ContainerSize::Large => style::large,
        ContainerSize::XLarge => style::xlarge,
    };

    let padding_class = if padding { style::padded } else { "" };

    rsx! {
        div { class: stylance::classes!(style::container, size_class, padding_class),
            {children}
        }
    }
}
