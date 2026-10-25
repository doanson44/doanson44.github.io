use std::rc::Rc;

use js_sys::Date;
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use crate::application::ports::TradingHistoryProvider;
use crate::domain::trading::TradingSnapshot;
use crate::domain::trading_history::{
    FundingSettlement, HistoricalCandle, TradingMarketHistory,
};

const PROXY_ENDPOINT: &str = "https://doanson44-forward-proxy.work-sontd.workers.dev/api/proxy";
const MEXC_FUTURES_API: &str = "https://contract.mexc.com/api/v1/contract";
const CANDLES_PER_REQUEST: i64 = 1_500;
const MAX_FUNDING_PAGES: u32 = 100;

/// Browser adapter for MEXC historical Futures candles and funding settlements.
#[derive(Debug, Clone, Copy, Default)]
pub struct MexcTradingHistoryApi;

impl TradingHistoryProvider for MexcTradingHistoryApi {
    fn load_history(
        &self,
        snapshot: &TradingSnapshot,
        on_result: Rc<dyn Fn(Result<TradingMarketHistory, String>)>,
    ) {
        let snapshot = snapshot.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = load_history_async(&snapshot).await;
            on_result(result);
        });
    }
}

async fn load_history_async(snapshot: &TradingSnapshot) -> Result<TradingMarketHistory, String> {
    let now_ms = Date::now() as i64;
    let mut history = TradingMarketHistory::default();

    for position in &snapshot.portfolio.positions {
        if position.opened_at_ms <= 0 {
            return Err(format!(
                "Cannot reconstruct {} history because its opening timestamp is missing. Close and reopen this legacy position to enable historical replay.",
                position.symbol
            ));
        }

        let symbol = position.symbol.trim().to_ascii_uppercase();
        let candles = fetch_candles(&symbol, position.opened_at_ms, now_ms).await?;
        let funding = fetch_funding_history(&symbol, position.opened_at_ms, now_ms).await?;

        if candles.is_empty() {
            return Err(format!("MEXC returned no fair-price candles for {symbol}."));
        }
        history.candles.insert(symbol.clone(), candles);
        history.funding.insert(symbol, funding);
    }

    Ok(history)
}

async fn fetch_candles(
    symbol: &str,
    opened_at_ms: i64,
    now_ms: i64,
) -> Result<Vec<HistoricalCandle>, String> {
    let mut candles = Vec::new();
    let mut chunk_start = (opened_at_ms / 1_000).max(0);
    let end_seconds = now_ms / 1_000;
    let duration_seconds = end_seconds.saturating_sub(chunk_start);
    // Use finer candles for recent trades and coarser candles for long-running
    // positions to stay within public API limits and avoid hundreds of requests.
    let (interval, interval_seconds) = if duration_seconds <= 24 * 60 * 60 {
        ("Min1", 60)
    } else if duration_seconds <= 14 * 24 * 60 * 60 {
        ("Min5", 5 * 60)
    } else if duration_seconds <= 90 * 24 * 60 * 60 {
        ("Min15", 15 * 60)
    } else if duration_seconds <= 365 * 24 * 60 * 60 {
        ("Min60", 60 * 60)
    } else {
        ("Hour4", 4 * 60 * 60)
    };
    let chunk_seconds = CANDLES_PER_REQUEST * interval_seconds;

    while chunk_start < end_seconds {
        let chunk_end = (chunk_start + chunk_seconds).min(end_seconds);
        let url = format!(
            "{MEXC_FUTURES_API}/kline/fair_price/{symbol}?interval={interval}&start={chunk_start}&end={chunk_end}"
        );
        let body = fetch_target(&url).await?;
        let mut parsed = parse_candles(&body, symbol)?;
        candles.append(&mut parsed);
        chunk_start = chunk_end.saturating_add(interval_seconds);
    }

    candles.sort_by_key(|candle| candle.time_ms);
    candles.dedup_by_key(|candle| candle.time_ms);
    Ok(candles)
}

async fn fetch_funding_history(
    symbol: &str,
    opened_at_ms: i64,
    now_ms: i64,
) -> Result<Vec<FundingSettlement>, String> {
    let mut settlements = Vec::new();
    let mut history_complete = false;

    for page in 1..=MAX_FUNDING_PAGES {
        let url = format!(
            "{MEXC_FUTURES_API}/funding_rate/history?symbol={symbol}&page_num={page}&page_size=1000"
        );
        let body = fetch_target(&url).await?;
        let (mut page_items, oldest_time) = parse_funding_page(&body, symbol)?;
        if page_items.is_empty() {
            history_complete = true;
            break;
        }

        settlements.extend(page_items.drain(..).filter(|item| {
            item.settle_time_ms > opened_at_ms && item.settle_time_ms <= now_ms
        }));

        if oldest_time.is_some_and(|timestamp| timestamp <= opened_at_ms) {
            history_complete = true;
            break;
        }
    }

    if !history_complete {
        return Err(format!(
            "Funding history for {symbol} exceeded the pagination limit before reaching the opening time."
        ));
    }

    settlements.sort_by_key(|item| item.settle_time_ms);
    settlements.dedup_by_key(|item| item.settle_time_ms);
    Ok(settlements)
}

async fn fetch_target(url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let headers = Headers::new().map_err(|error| format!("Proxy headers failed: {}", js_error(&error)))?;
    headers
        .set("Accept", "application/json")
        .map_err(|error| format!("Proxy Accept header failed: {}", js_error(&error)))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|error| format!("Proxy Content-Type header failed: {}", js_error(&error)))?;

    let body = serde_json::json!({ "targetUrl": url, "method": "GET" }).to_string();
    let options = RequestInit::new();
    options.set_method("POST");
    options.set_mode(RequestMode::Cors);
    options.set_headers(&headers);
    options.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(PROXY_ENDPOINT, &options)
        .map_err(|error| format!("Proxy request creation failed: {}", js_error(&error)))?;
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|error| format!("Proxy request failed: {}", js_error(&error)))?;
    let response: Response = response
        .dyn_into()
        .map_err(|_| "Proxy returned an invalid response".to_string())?;
    if !response.ok() {
        return Err(format!("MEXC history request returned HTTP {}", response.status()));
    }
    JsFuture::from(
        response
            .text()
            .map_err(|error| format!("Reading MEXC response failed: {}", js_error(&error)))?,
    )
    .await
    .map_err(|error| format!("Reading MEXC response failed: {}", js_error(&error)))?
    .as_string()
    .ok_or_else(|| "MEXC history response was not text".to_string())
}

fn parse_candles(raw: &str, symbol: &str) -> Result<Vec<HistoricalCandle>, String> {
    let payload: Value = serde_json::from_str(raw)
        .map_err(|error| format!("Could not decode {symbol} candle history: {error}"))?;
    if payload.get("success").and_then(Value::as_bool) == Some(false) {
        return Err(format!("MEXC rejected fair-price history for {symbol}."));
    }

    let data = payload.get("data").unwrap_or(&payload);
    let mut output = Vec::new();

    if let Some(object) = data.as_object() {
        let times = object.get("time").and_then(Value::as_array);
        let highs = object.get("high").and_then(Value::as_array);
        let lows = object.get("low").and_then(Value::as_array);
        let closes = object.get("close").and_then(Value::as_array);
        if let (Some(times), Some(highs), Some(lows), Some(closes)) = (times, highs, lows, closes) {
            for index in 0..times.len().min(highs.len()).min(lows.len()).min(closes.len()) {
                if let (Some(time), Some(high), Some(low), Some(close)) = (
                    number(&times[index]),
                    number(&highs[index]),
                    number(&lows[index]),
                    number(&closes[index]),
                ) {
                    output.push(HistoricalCandle {
                        time_ms: normalize_timestamp(time),
                        high,
                        low,
                        close,
                    });
                }
            }
        }
    } else if let Some(rows) = data.as_array() {
        for row in rows {
            let time = row.get("time").and_then(number).or_else(|| row.get(0).and_then(number));
            let high = row.get("high").and_then(number).or_else(|| row.get(3).and_then(number));
            let low = row.get("low").and_then(number).or_else(|| row.get(4).and_then(number));
            let close = row.get("close").and_then(number).or_else(|| row.get(5).and_then(number));
            if let (Some(time), Some(high), Some(low), Some(close)) = (time, high, low, close) {
                output.push(HistoricalCandle {
                    time_ms: normalize_timestamp(time),
                    high,
                    low,
                    close,
                });
            }
        }
    }

    Ok(output.into_iter().filter(|candle| {
        candle.time_ms > 0
            && candle.high.is_finite()
            && candle.low.is_finite()
            && candle.close.is_finite()
            && candle.low > 0.0
            && candle.high >= candle.low
            && candle.close > 0.0
    }).collect())
}

fn parse_funding_page(
    raw: &str,
    symbol: &str,
) -> Result<(Vec<FundingSettlement>, Option<i64>), String> {
    let payload: Value = serde_json::from_str(raw)
        .map_err(|error| format!("Could not decode {symbol} funding history: {error}"))?;
    if payload.get("success").and_then(Value::as_bool) == Some(false) {
        return Err(format!("MEXC rejected funding history for {symbol}."));
    }

    let data = payload.get("data").unwrap_or(&payload);
    let rows = data.as_array().or_else(|| {
        data.get("resultList").and_then(Value::as_array)
    }).or_else(|| {
        data.get("data").and_then(Value::as_array)
    });
    let Some(rows) = rows else {
        return Err(format!("MEXC funding history response for {symbol} has an unknown format."));
    };

    let mut output = Vec::new();
    let mut oldest_time: Option<i64> = None;
    for row in rows {
        let time = row.get("settleTime").and_then(number)
            .or_else(|| row.get("settle_time").and_then(number));
        let rate = row.get("fundingRate").and_then(number)
            .or_else(|| row.get("funding_rate").and_then(number));
        if let (Some(time), Some(rate)) = (time, rate) {
            let settle_time_ms = normalize_timestamp(time);
            oldest_time = Some(oldest_time.map_or(settle_time_ms, |oldest| oldest.min(settle_time_ms)));
            if settle_time_ms > 0 && rate.is_finite() {
                output.push(FundingSettlement { settle_time_ms, funding_rate: rate });
            }
        }
    }
    if !rows.is_empty() && output.is_empty() {
        return Err(format!(
            "MEXC funding history response for {symbol} contained no parseable settlement records."
        ));
    }
    Ok((output, oldest_time))
}

fn number(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.as_str()?.parse::<f64>().ok())
}

fn normalize_timestamp(timestamp: f64) -> i64 {
    if timestamp.abs() < 100_000_000_000.0 {
        (timestamp * 1_000.0) as i64
    } else {
        timestamp as i64
    }
}

fn js_error(error: &JsValue) -> String {
    error.as_string().unwrap_or_else(|| "Unknown browser error".into())
}

