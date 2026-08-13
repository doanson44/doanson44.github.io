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

/// Read a value from local browser storage.
pub fn storage_get(key: &str) -> Option<String> {
    let storage = web_sys::window()?.local_storage().ok()??;
    storage.get_item(key).ok().flatten()
}

/// Write a value to local browser storage.
pub fn storage_set(key: &str, value: &str) -> Result<(), String> {
    let storage = web_sys::window()
        .ok_or_else(|| "Browser window is unavailable.".to_string())?
        .local_storage()
        .map_err(|_| "Browser storage is unavailable.".to_string())?
        .ok_or_else(|| "Browser storage is unavailable.".to_string())?;
    storage
        .set_item(key, value)
        .map_err(|_| "Failed to save tool input.".to_string())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "__copy_svg_as_png", catch)]
    async fn copy_svg_as_png_js(svg_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "__copy_preview_as_html", catch)]
    async fn copy_preview_as_html_js(element_id: &str) -> Result<JsValue, JsValue>;

    /// Toggle between dark and light theme. Returns the new theme name.
    #[wasm_bindgen(js_name = "__toggle_theme")]
    pub fn toggle_theme_js() -> String;
}

/// Copy text to the system clipboard.
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let navigator = get_window().navigator();
    let clipboard = navigator.clipboard();
    let promise = clipboard.write_text(text);
    let _ = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| {
            e.as_string()
                .unwrap_or_else(|| "Clipboard write failed".into())
        })?;
    Ok(())
}

/// Copy the rendered preview as rich HTML with a plain-text fallback.
pub async fn copy_preview_as_html(element_id: &str) -> Result<(), String> {
    match copy_preview_as_html_js(element_id).await {
        Ok(res)
            if res
                .as_string()
                .is_some_and(|json| json.contains("\"ok\":true")) =>
        {
            Ok(())
        }
        Ok(_) => Err("Failed to copy preview as rich HTML".to_string()),
        Err(e) => Err(e
            .as_string()
            .unwrap_or_else(|| "Failed to copy preview".into())),
    }
}

/// Copy an SVG element as a PNG image to the clipboard using the global JS interop function.
pub async fn copy_svg_as_png(svg_id: &str) -> Result<(), String> {
    match copy_svg_as_png_js(svg_id).await {
        Ok(res)
            if res
                .as_string()
                .is_some_and(|json| json.contains("\"ok\":true")) =>
        {
            Ok(())
        }
        Ok(_) => Err("JS function returned an error".to_string()),
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
