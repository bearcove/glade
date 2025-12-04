//! Utility functions for working with Dioxus events

/// Extension trait for clipboard events to get the underlying DataTransfer
#[cfg(target_arch = "wasm32")]
pub trait ClipboardEventExt {
    /// Get the clipboard data from a paste event
    fn clipboard_data(&self) -> Option<web_sys::DataTransfer>;

    /// Get plain text from a paste event
    fn text(&self) -> Option<String> {
        self.clipboard_data()
            .and_then(|data| data.get_data("text/plain").ok())
    }
}

#[cfg(target_arch = "wasm32")]
impl ClipboardEventExt for dioxus::core::Event<dioxus::html::ClipboardData> {
    fn clipboard_data(&self) -> Option<web_sys::DataTransfer> {
        use dioxus_web::WebEventExt;
        use wasm_bindgen::JsCast;

        let web_evt: web_sys::Event = self.as_web_event();
        web_evt
            .dyn_ref::<web_sys::ClipboardEvent>()
            .and_then(|evt| evt.clipboard_data())
    }
}

/// Extension trait for MountedData to work with HTML input elements
#[cfg(target_arch = "wasm32")]
pub trait MountedInputExt {
    /// Try to get this mounted element as an HtmlInputElement
    fn as_input(&self) -> Option<web_sys::HtmlInputElement>;

    /// Select all text in the input element
    fn select(&self) {
        if let Some(input) = self.as_input() {
            let _ = input.select();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl MountedInputExt for dioxus::html::MountedData {
    fn as_input(&self) -> Option<web_sys::HtmlInputElement> {
        use wasm_bindgen::JsCast;

        self.downcast::<web_sys::Element>()
            .cloned()
            .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    }
}
