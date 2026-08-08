//! Browser utility functions.
//!
//! Wraps `web-sys` calls behind safe, idiomatic Rust functions.
//! All browser-specific concerns are isolated here, keeping domain
//! and application logic free of browser API dependencies.

use wasm_bindgen::JsCast;

/// Get the browser window object.
pub fn get_window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

/// Get the browser document object.
pub fn get_document() -> web_sys::Document {
    get_window()
        .document()
        .expect("should have a document on window")
}

/// Log a message to the browser console.
pub fn console_log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

/// Get an element by ID, returning a typed element.
pub fn get_element_by_id<T: wasm_bindgen::JsCast>(id: &str) -> Option<T> {
    get_document()
        .get_element_by_id(id)?
        .dyn_into::<T>()
        .ok()
}
