use std::{collections::HashMap, rc::Rc};

use js_sys::Date;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Storage;

use crate::application::ports::FundingRateProvider;
use crate::domain::funding::FundingRateSnapshot;

const FUNDING_ENDPOINT: &str = "https://fapi.binance.com/fapi/v1/premiumIndex";
const CACHE_KEY: &str = "socket.funding-rate-cache.v4";
const CACHE_TTL_MS: f64 = 60.0 * 60.0 * 1000.0;

#[derive(Debug, Deserialize)]
struct ApiFundingRate {
    symbol: String,
    #[serde(rename = "lastFundingRate")]
    funding_rate: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedSnapshot {
    fetched_at_ms: f64,
    rates: HashMap<String, f64>,
}

/// Browser implementation of the all-market funding rate provider.
#[derive(Debug, Default, Clone, Copy)]
pub struct FundingRateApi;

impl FundingRateProvider for FundingRateApi {
    fn load_cached_or_fetch(&self, on_result: Rc<dyn Fn(Result<FundingRateSnapshot, String>)>) {
        if let Some(snapshot) = load_cache() {
            on_result(Ok(snapshot));
            return;
        }

        let callback = on_result.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = fetch_snapshot().await;
            if let Ok(snapshot) = &result {
                let _ = save_cache(snapshot);
            }
            callback(result);
        });
    }
}

async fn fetch_snapshot() -> Result<FundingRateSnapshot, String> {
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let response = JsFuture::from(window.fetch_with_str(FUNDING_ENDPOINT))
        .await
        .map_err(|error| format!("Funding rate request failed: {}", js_error(&error)))?;
    let response: web_sys::Response = response
        .dyn_into()
        .map_err(|_| "Funding rate response is invalid".to_string())?;

    if !response.ok() {
        return Err(format!(
            "Funding rate request returned HTTP {}",
            response.status()
        ));
    }

    let text =
        JsFuture::from(response.text().map_err(|error| {
            format!("Failed to read funding rate response: {}", js_error(&error))
        })?)
        .await
        .map_err(|error| format!("Failed to read funding rate response: {}", js_error(&error)))?
        .as_string()
        .ok_or_else(|| "Funding rate response was not text".to_string())?;

    let payload: Vec<ApiFundingRate> = serde_json::from_str(&text)
        .map_err(|error| format!("Failed to decode funding rate response: {error}"))?;

    let rates = payload
        .into_iter()
        .filter(|item| item.symbol.ends_with("USDT"))
        .filter_map(|item| {
            item.funding_rate
                .parse::<f64>()
                .ok()
                .map(|rate| (item.symbol.replace("USDT", "_USDT"), rate))
        })
        .collect();

    Ok(FundingRateSnapshot::new(rates))
}

fn load_cache() -> Option<FundingRateSnapshot> {
    let storage = storage()?;
    let raw = storage.get_item(CACHE_KEY).ok().flatten()?;
    let cached = serde_json::from_str::<CachedSnapshot>(&raw).ok()?;
    let age = Date::now() - cached.fetched_at_ms;
    if !age.is_finite() || !(0.0..CACHE_TTL_MS).contains(&age) {
        let _ = storage.remove_item(CACHE_KEY);
        return None;
    }
    Some(FundingRateSnapshot::new(cached.rates))
}

fn save_cache(snapshot: &FundingRateSnapshot) -> Result<(), String> {
    let storage = storage().ok_or_else(|| "Browser storage is unavailable".to_string())?;
    let cached = CachedSnapshot {
        fetched_at_ms: Date::now(),
        rates: snapshot.rates.clone(),
    };
    let value = serde_json::to_string(&cached)
        .map_err(|error| format!("Failed to encode funding rate cache: {error}"))?;
    storage
        .set_item(CACHE_KEY, &value)
        .map_err(|error| format!("Failed to store funding rate cache: {}", js_error(&error)))
}

fn storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn js_error(error: &JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Unknown browser error".into())
}
