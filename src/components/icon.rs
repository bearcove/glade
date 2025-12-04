//! Icon component for displaying SVG icons

use dioxus::prelude::*;

stylance::import_style!(style, "icon.module.scss");

/// Size variants for Icon
#[derive(Clone, Copy, PartialEq)]
pub enum IconSize {
    /// Small icon (16px)
    Small,
    /// Medium icon (20px)
    Medium,
    /// Large icon (24px)
    Large,
    /// Extra large icon (32px)
    XL,
}

/// Basic icon component that renders SVG content
///
/// For file type icons, use the grove-icon web component instead.
/// This is a simple wrapper for inline SVG icons.
#[component]
pub fn Icon(
    #[props(default = IconSize::Medium)] size: IconSize,
    /// Raw SVG content to render
    svg: String,
) -> Element {
    let size_class = match size {
        IconSize::Small => style::small,
        IconSize::Medium => "",
        IconSize::Large => style::large,
        IconSize::XL => style::xl,
    };

    rsx! {
        span {
            class: stylance::classes!(style::icon, size_class),
            dangerous_inner_html: svg,
        }
    }
}
