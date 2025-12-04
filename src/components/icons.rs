//! Lucide-based icon components for glade
//!
//! Icons from https://lucide.dev/ (ISC License)
//! SVGs are included at compile time and rendered via dangerous_inner_html.

use dioxus::prelude::*;

/// Helper macro to create icon components
macro_rules! icon {
    ($name:ident, $file:literal) => {
        #[component]
        pub fn $name(#[props(default)] class: Option<&'static str>) -> Element {
            rsx! {
                span {
                    class: class,
                    dangerous_inner_html: include_str!(concat!("icons/", $file)),
                }
            }
        }
    };
}

// Icons used by glade-dioxus components
icon!(IconAlertCircle, "circle-alert.svg");
icon!(IconCheck, "check.svg");
icon!(IconChevronDown, "chevron-down.svg");
icon!(IconChevronLeft, "chevron-left.svg");
icon!(IconChevronRight, "chevron-right.svg");
icon!(IconCircleCheck, "circle-check.svg");
icon!(IconCircleX, "circle-x.svg");
icon!(IconExternalLink, "external-link.svg");
icon!(IconFilter, "filter.svg");
icon!(IconInfo, "info.svg");
icon!(IconLoader, "loader.svg");
icon!(IconMenu, "menu.svg");
icon!(IconMinus, "minus.svg");
icon!(IconPlus, "plus.svg");
icon!(IconSearch, "search.svg");
icon!(IconTriangleAlert, "triangle-alert.svg");
icon!(IconX, "x.svg");

// Icons used by bearcove
icon!(IconArchive, "archive.svg");
icon!(IconBuilding2, "building-2.svg");
icon!(IconClipboardList, "clipboard-list.svg");
icon!(IconFileText, "file-text.svg");
icon!(IconGithub, "github.svg");
icon!(IconGlobe, "globe.svg");
icon!(IconMapPin, "map-pin.svg");
icon!(IconMic, "mic.svg");
icon!(IconServer, "server.svg");
icon!(IconUser, "user.svg");
icon!(IconYoutube, "youtube.svg");
