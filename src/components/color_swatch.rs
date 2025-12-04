//! Color swatch inline component for displaying colors

use dioxus::prelude::*;

stylance::import_style!(style, "color_swatch.module.scss");

/// Format for displaying the color value
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorFormat {
    /// Hex format (#ff5500)
    #[default]
    Hex,
    /// RGB format (rgb(255, 85, 0))
    Rgb,
    /// HSL format (hsl(20, 100%, 50%))
    Hsl,
}

/// Inline color swatch with a colored square and text value
#[component]
pub fn ColorSwatch(
    /// The color value (CSS color string)
    color: String,
    /// Display format for the color text
    #[props(default)]
    format: ColorFormat,
    /// Whether clicking copies the color value
    #[props(default = true)]
    copyable: bool,
    /// Custom label (overrides auto-formatted text)
    label: Option<String>,
) -> Element {
    let color_for_style = color.clone();
    let color_for_display = color.clone();

    let display_text = label.unwrap_or_else(|| {
        // For now, just display the color as-is
        // A full implementation would parse and convert between formats
        match format {
            ColorFormat::Hex | ColorFormat::Rgb | ColorFormat::Hsl => color_for_display.clone(),
        }
    });

    let class = if copyable {
        stylance::classes!(style::swatch, style::copyable)
    } else {
        style::swatch.to_string()
    };

    rsx! {
        span {
            class: class,
            onclick: move |_| {
                if copyable {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(window) = web_sys::window() {
                            let clipboard = window.navigator().clipboard();
                            let _ = clipboard.write_text(&color);
                        }
                    }
                }
            },
            span {
                class: style::color,
                style: "background-color: {color_for_style}",
            }
            span { class: style::value, "{display_text}" }
        }
    }
}
