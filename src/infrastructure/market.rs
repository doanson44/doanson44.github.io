use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::application::ports::MarketClient;

const MARKET_URL: &str = "https://cafef.vn/du-lieu/ajax/mobile/smart/ajaxbandothitruong.ashx";

/// Browser implementation of the direct CafeF market-data client.
#[derive(Debug, Default, Clone, Copy)]
pub struct MarketApi;

impl MarketClient for MarketApi {
    fn fetch(&self, on_result: Rc<dyn Fn(Result<String, String>)>) {
        wasm_bindgen_futures::spawn_local(async move {
            on_result(fetch_market_data().await);
        });
    }
}

async fn fetch_market_data() -> Result<String, String> {
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;

    let options = RequestInit::new();
    options.set_method("GET");
    options.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(MARKET_URL, &options)
        .map_err(|error| format!("Failed to create market request: {}", js_error(&error)))?;

    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|error| format!("Market request failed: {}", js_error(&error)))?;

    let response: Response = response
        .dyn_into()
        .map_err(|_| "Market response is invalid".to_string())?;

    if !response.ok() {
        return Err(format!(
            "CafeF market request returned HTTP {}",
            response.status()
        ));
    }

    JsFuture::from(
        response
            .text()
            .map_err(|error| format!("Failed to read market response: {}", js_error(&error)))?,
    )
    .await
    .map_err(|error| format!("Failed to read market response: {}", js_error(&error)))?
    .as_string()
    .ok_or_else(|| "CafeF market response was not text".to_string())
}

fn js_error(error: &JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Unknown browser error".into())
}
