mod normalize;
pub use normalize::normalize_mermaid_source;

use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Calls the global `__mermaid_render(id, code)` function defined in index.html.
    /// Returns a JSON string with `{ ok: bool, svg?: string, error?: string }`.
    #[wasm_bindgen(js_name = "__mermaid_render", catch)]
    async fn mermaid_render_js(id: &str, code: &str) -> Result<JsValue, JsValue>;
}

/// Result of a Mermaid rendering attempt.
#[derive(Debug, Clone)]
pub enum MermaidResult {
    /// Successfully rendered SVG.
    Success(String),
    /// Rendering failed with an error message.
    Error(String),
}

/// Response structure from the JS mermaid render function.
#[derive(Deserialize)]
struct MermaidResponse {
    ok: bool,
    svg: Option<String>,
    error: Option<String>,
}

/// Render a Mermaid diagram definition into SVG.
///
/// This function calls the Mermaid.js library via JavaScript interop.
/// The Mermaid library is loaded via CDN in `index.html` and exposed
/// through a global `__mermaid_render` function.
///
/// # Arguments
/// * `id` - Unique identifier for this diagram (used by Mermaid internally)
/// * `code` - The Mermaid diagram definition
///
/// # Returns
/// `MermaidResult::Success(svg)` on success, `MermaidResult::Error(msg)` on failure.
///
/// Invalid Mermaid syntax produces an error result without panicking,
/// ensuring the rest of the preview remains functional.
pub async fn render_mermaid(id: &str, code: &str) -> MermaidResult {
    // Normalize the source before passing to Mermaid.js.
    // The original `code` is never mutated; `normalized` is a local value used
    // only for rendering. Copy/export functionality continues to use `code`.
    let normalized = normalize_mermaid_source(code);
    match mermaid_render_js(id, &normalized).await {
        Ok(js_value) => {
            let json_str = match js_value.as_string() {
                Some(s) => s,
                None => return MermaidResult::Error("Mermaid returned non-string value".into()),
            };

            match serde_json::from_str::<MermaidResponse>(&json_str) {
                Ok(response) => {
                    if response.ok {
                        MermaidResult::Success(response.svg.unwrap_or_default())
                    } else {
                        MermaidResult::Error(
                            response
                                .error
                                .unwrap_or_else(|| "Unknown Mermaid error".into()),
                        )
                    }
                }
                Err(e) => MermaidResult::Error(format!("Failed to parse Mermaid response: {e}")),
            }
        }
        Err(e) => {
            let msg = e
                .as_string()
                .unwrap_or_else(|| format!("Mermaid rendering failed: {e:?}"));
            MermaidResult::Error(msg)
        }
    }
}
