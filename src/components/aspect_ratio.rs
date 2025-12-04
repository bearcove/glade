//! AspectRatio component for maintaining content aspect ratios

use dioxus::prelude::*;

stylance::import_style!(style, "aspect_ratio.module.scss");

/// Common aspect ratio presets
#[derive(Clone, Copy, PartialEq)]
pub enum AspectRatioPreset {
    /// 1:1 square
    Square,
    /// 16:9 widescreen video
    Video,
    /// 4:3 classic video/photo
    Classic,
    /// 3:2 photo
    Photo,
    /// 21:9 ultrawide
    Ultrawide,
    /// 9:16 portrait video (TikTok, Reels)
    Portrait,
    /// Custom ratio (width / height)
    Custom(f64),
}

impl AspectRatioPreset {
    fn to_ratio(self) -> f64 {
        match self {
            Self::Square => 1.0,
            Self::Video => 16.0 / 9.0,
            Self::Classic => 4.0 / 3.0,
            Self::Photo => 3.0 / 2.0,
            Self::Ultrawide => 21.0 / 9.0,
            Self::Portrait => 9.0 / 16.0,
            Self::Custom(ratio) => ratio,
        }
    }
}

impl Default for AspectRatioPreset {
    fn default() -> Self {
        Self::Video
    }
}

/// A container that maintains a specific aspect ratio
#[component]
pub fn AspectRatio(
    /// The aspect ratio to maintain (width / height)
    #[props(default)]
    ratio: AspectRatioPreset,
    /// Content to render inside the container
    children: Element,
    /// Additional CSS class
    #[props(optional, into)]
    class: Option<String>,
) -> Element {
    let ratio_value = ratio.to_ratio();
    let padding_bottom = (1.0 / ratio_value) * 100.0;

    rsx! {
        div {
            class: stylance::classes!(style::aspect_ratio, class.as_deref().unwrap_or("")),
            style: format!("padding-bottom: {padding_bottom:.4}%;"),
            div { class: style::content,
                {children}
            }
        }
    }
}
