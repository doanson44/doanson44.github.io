//! Browser utility functions.
//!
//! Wraps `web-sys` calls behind safe, idiomatic Rust functions.
//! All browser-specific concerns are isolated here, keeping domain
//! and application logic free of browser API dependencies.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Get the browser window object.
pub fn get_window() -> web_sys::Window {
    web_sys::window().expect("should have a window in this context")
}

/// Get the browser document object.
pub fn get_document() -> web_sys::Document {
    get_window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "__copy_svg_as_png", catch)]
    async fn copy_svg_as_png_js(svg_id: &str) -> Result<JsValue, JsValue>;
}

/// Copy text to the system clipboard.
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let navigator = get_window().navigator();
    let clipboard = navigator.clipboard();

    // We can use web_sys::Clipboard if available
    let promise = clipboard.write_text(text);
    let _ = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| {
            e.as_string()
                .unwrap_or_else(|| "Clipboard write failed".into())
        })?;

    Ok(())
}

/// Copy an SVG element as a PNG image to the clipboard using the global JS interop function.
pub async fn copy_svg_as_png(svg_id: &str) -> Result<(), String> {
    match copy_svg_as_png_js(svg_id).await {
        Ok(res) => {
            let json = res.as_string().unwrap_or_default();
            if json.contains("\"ok\":true") {
                Ok(())
            } else {
                Err("JS function returned an error".to_string())
            }
        }
        Err(e) => Err(e
            .as_string()
            .unwrap_or_else(|| "Failed to copy image".into())),
    }
}

/// Log a message to the browser console.
pub fn console_log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

/// Get an element by ID, returning a typed element.
pub fn get_element_by_id<T: wasm_bindgen::JsCast>(id: &str) -> Option<T> {
    get_document().get_element_by_id(id)?.dyn_into::<T>().ok()
}
