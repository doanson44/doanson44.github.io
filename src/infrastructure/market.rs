use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::application::ports::{MarketClient, MarketPinStore};

const DESKTOP_MARKET_URL: &str =
    "https://cafef.vn/du-lieu/ajax/mobile/smart/ajaxbandothitruong.ashx";
const MOBILE_MARKET_URL: &str =
    "https://m.cafef.vn/du-lieu/ajax/mobile/smart/ajaxbandothitruong.ashx";
const PIN_CACHE_KEY: &str = "market.pinned-symbols.v1";

/// Browser implementation of the direct CafeF market-data client.
#[derive(Debug, Default, Clone, Copy)]
pub struct MarketApi;

impl MarketClient for MarketApi {
    fn fetch(&self, on_result: Rc<dyn Fn(Result<String, String>)>) {
        wasm_bindgen_futures::spawn_local(async move {
            on_result(fetch_market_data().await);
        });
    }

    fn fetch_url(&self, target_url: &str, on_result: Rc<dyn Fn(Result<String, String>)>) {
        let target_url = target_url.trim().to_string();
        wasm_bindgen_futures::spawn_local(async move {
            on_result(fetch_market_url(&target_url).await);
        });
    }
}

impl MarketPinStore for MarketApi {
    fn load(&self) -> Result<Vec<String>, String> {
        let raw = crate::infrastructure::browser::storage_get(PIN_CACHE_KEY);
        match raw {
            Some(value) => serde_json::from_str(&value)
                .map_err(|error| format!("Failed to decode market pin cache: {error}")),
            None => Ok(Vec::new()),
        }
    }

    fn save(&self, symbols: &[String]) -> Result<(), String> {
        let value = serde_json::to_string(symbols)
            .map_err(|error| format!("Failed to encode market pin cache: {error}"))?;
        crate::infrastructure::browser::storage_set(PIN_CACHE_KEY, &value)
            .map_err(|error| format!("Failed to store market pin cache: {error}"))
    }
}

async fn fetch_market_url(target_url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let options = RequestInit::new();
    options.set_method("GET");
    options.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(target_url, &options)
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

async fn fetch_market_data() -> Result<String, String> {
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;

    let options = RequestInit::new();
    options.set_method("GET");
    options.set_mode(RequestMode::Cors);

    let market_url = if crate::infrastructure::browser::is_mobile_device() {
        MOBILE_MARKET_URL
    } else {
        DESKTOP_MARKET_URL
    };

    let request = Request::new_with_str_and_init(market_url, &options)
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
