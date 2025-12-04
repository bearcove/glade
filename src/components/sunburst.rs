//! Sunburst loading indicator - rotating rays

use dioxus::prelude::*;

stylance::import_style!(style, "sunburst.module.scss");

/// Size variants for the sunburst
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SunburstSize {
    /// Small (12px)
    Small,
    /// Medium (16px) - default
    #[default]
    Medium,
    /// Large (24px)
    Large,
}

/// Sunburst loading indicator - rotating sun-ray icon
#[component]
pub fn Sunburst(
    /// Size of the sunburst
    #[props(default)]
    size: SunburstSize,
) -> Element {
    let size_class = match size {
        SunburstSize::Small => style::small,
        SunburstSize::Medium => style::medium,
        SunburstSize::Large => style::large,
    };

    rsx! {
        span { class: stylance::classes!(style::sunburst, size_class),
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83" }
            }
        }
    }
}
