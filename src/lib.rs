//! Glade - A Dioxus component library with stylance-based CSS modules

use dioxus::prelude::*;

pub mod components;
pub mod hooks;
pub mod utils;

pub use components::*;
pub use hooks::{use_callback_channel, CallbackSender};

// NOTE: asset!() macro requires CARGO_MANIFEST_DIR which isn't set in Buck2 builds.
// These are disabled for now until we have a Buck2-compatible asset bundling solution.
#[cfg(not(feature = "buck2"))]
/// Base CSS with color variables (Radix Green scale)
pub static GLADE_BASE_CSS: Asset = asset!("/css/base.css");

#[cfg(not(feature = "buck2"))]
/// CSS compiled via the stylance CLI, collecting all scoped styles
pub static GLADE_STYLANCE_CSS: Asset = asset!("/css/stylance.scss");
