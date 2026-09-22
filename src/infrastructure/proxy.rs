use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use crate::application::ports::ProxyClient;

const PROXY_ENDPOINT: &str =
    "https://doanson44-forward-proxy.work-sontd.workers.dev/api/proxy";

/// Browser adapter for the external CORS/forward proxy.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProxyApi;

impl ProxyClient for ProxyApi {
    fn fetch(&self, target_url: &str, on_result: Rc<dyn Fn(Result<String, String>)>) {
        let target_url = target_url.trim().to_string();

        wasm_bindgen_futures::spawn_local(async move {
            let result = fetch_text(&target_url).await;
            on_result(result);
        });
    }
}

async fn fetch_text(target_url: &str) -> Result<String, String> {
    if !(target_url.starts_with("https://") || target_url.starts_with("http://")) {
        return Err("Target URL must use http:// or https://.".into());
    }

    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let headers = Headers::new()
        .map_err(|error| format!("Failed to create proxy headers: {}", js_error(&error)))?;

    headers
        .set("Accept", "application/json")
        .map_err(|error| format!("Failed to set Accept header: {}", js_error(&error)))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|error| format!("Failed to set Content-Type header: {}", js_error(&error)))?;

    let body = serde_json::json!({
        "targetUrl": target_url,
        "method": "GET",
    })
    .to_string();

    let options = RequestInit::new();
    options.set_method("POST");
    options.set_mode(RequestMode::Cors);
    options.set_headers(&headers);
    options.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(PROXY_ENDPOINT, &options)
        .map_err(|error| format!("Failed to create proxy request: {}", js_error(&error)))?;

    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|error| format!("Proxy request failed: {}", js_error(&error)))?;

    let response: Response = response
        .dyn_into()
        .map_err(|_| "Proxy response is invalid".to_string())?;

    if !response.ok() {
        return Err(format!(
            "Proxy request returned HTTP {}",
            response.status()
        ));
    }

    JsFuture::from(
        response
            .text()
            .map_err(|error| format!("Failed to read proxy response: {}", js_error(&error)))?,
    )
    .await
    .map_err(|error| format!("Failed to read proxy response: {}", js_error(&error)))?
    .as_string()
    .ok_or_else(|| "Proxy response was not text".to_string())
}

fn js_error(error: &JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Unknown browser error".into())
}
