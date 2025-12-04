//! Link component with optional external indicator

use dioxus::prelude::*;

use crate::IconExternalLink;

stylance::import_style!(style, "link.module.scss");

/// Link variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LinkVariant {
    /// Default link style with accent color
    #[default]
    Default,
    /// Muted/subtle link
    Muted,
    /// Inherit color from parent
    Inherit,
}

/// A styled link component with optional external icon
#[component]
pub fn Link(
    /// The URL to link to
    href: String,
    /// Link variant
    #[props(default)]
    variant: LinkVariant,
    /// Whether this is an external link (adds external icon and target="_blank")
    #[props(default = false)]
    external: bool,
    /// Whether to show underline
    #[props(default = true)]
    underline: bool,
    /// Link content
    children: Element,
) -> Element {
    let variant_class = match variant {
        LinkVariant::Default => style::default,
        LinkVariant::Muted => style::muted,
        LinkVariant::Inherit => style::inherit,
    };

    let underline_class = if underline { style::underline } else { "" };

    rsx! {
        a {
            href: "{href}",
            class: stylance::classes!(style::link, variant_class, underline_class),
            target: if external { "_blank" } else { "" },
            rel: if external { "noopener noreferrer" } else { "" },
            {children}
            if external {
                span { class: style::icon, IconExternalLink {} }
            }
        }
    }
}
