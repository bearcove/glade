//! Avatar component for displaying user images or initials

use dioxus::prelude::*;

stylance::import_style!(style, "avatar.module.scss");

/// Size variants for Avatar component
#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarSize {
    /// Small avatar (24px)
    Small,
    /// Medium avatar (32px, default)
    #[default]
    Medium,
    /// Large avatar (48px)
    Large,
    /// Extra large avatar (64px)
    XLarge,
}

/// Avatar component - displays a user image or fallback initials
#[component]
pub fn Avatar(
    #[props(default)] size: AvatarSize,
    /// Image URL
    #[props(default)]
    src: String,
    /// Alt text for the image
    #[props(default)]
    alt: String,
    /// Fallback initials (shown when no image)
    #[props(default)]
    initials: String,
) -> Element {
    let size_class = match size {
        AvatarSize::Small => style::small,
        AvatarSize::Medium => "",
        AvatarSize::Large => style::large,
        AvatarSize::XLarge => style::xlarge,
    };

    rsx! {
        div { class: stylance::classes!(style::avatar, size_class),
            if !src.is_empty() {
                img {
                    class: style::image,
                    src: "{src}",
                    alt: "{alt}",
                }
            } else if !initials.is_empty() {
                span { class: style::initials, "{initials}" }
            }
        }
    }
}

/// AvatarGroup - displays multiple avatars with overlap
#[component]
pub fn AvatarGroup(
    #[props(default)] size: AvatarSize,
    /// Maximum number of avatars to show before "+N" indicator
    #[props(default = 4)]
    max: usize,
    children: Element,
) -> Element {
    let size_class = match size {
        AvatarSize::Small => style::group_small,
        AvatarSize::Medium => "",
        AvatarSize::Large => style::group_large,
        AvatarSize::XLarge => style::group_xlarge,
    };

    rsx! {
        div { class: stylance::classes!(style::group, size_class),
            {children}
        }
    }
}
