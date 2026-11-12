use std::rc::Rc;

use leptos::prelude::*;
use serde_json::{json, Value};

use crate::application::services::market::MarketService;
use crate::application::services::proxy::ProxyService;
use crate::application::services::technical_analysis::TechnicalAnalysisService;
use crate::domain::market::MarketPriceHistory;
use crate::domain::technical_analysis::AnalysisResult;
use crate::infrastructure::browser;
use crate::infrastructure::market::MarketApi;
use crate::infrastructure::proxy::ProxyApi;

const MEXC_REST_BASE: &str = "https://api.mexc.com/api/v1/contract";

#[derive(Clone, Copy)]
pub struct MarketDataState {
    pub output: RwSignal<String>,
}

type CryptoAnalysisCallback =
    Rc<dyn Fn(Result<(AnalysisResult, MarketPriceHistory), String>)>;

impl MarketDataState {
    pub fn new() -> Self {
        Self {
            output: RwSignal::new(json!({"status":"loading"}).to_string()),
        }
    }

    pub fn load_coin(&self, requested_symbol: &str) {
        let symbol = match normalize_coin_symbol(requested_symbol) {
            Ok(symbol) => symbol,
            Err(message) => {
                self.set_error("validation", message);
                return;
            }
        };

        let output = self.output;
        let ticker_url = format!("{MEXC_REST_BASE}/ticker?symbol={symbol}");
        ProxyService::new(ProxyApi).fetch_raw(
            &ticker_url,
            Rc::new(move |result| {
                let ticker = match result.and_then(|raw| parse_ticker_response(&raw)) {
                    Ok(ticker) => ticker,
                    Err(message) => {
                        output.set(error_json("ticker", &message));
                        return;
                    }
                };

                let four_hour_output = output;
                let symbol_for_four_hour = symbol.clone();
                Self::fetch_crypto_analysis(
                    &symbol,
                    "4h",
                    Rc::new(move |four_hour_result| {
                        let (four_hour_analysis, _) = match four_hour_result {
                            Ok(result) => result,
                            Err(message) => {
                                four_hour_output.set(error_json("analysis_4h", &message));
                                return;
                            }
                        };

                        let daily_output = four_hour_output;
                        let symbol_for_daily = symbol_for_four_hour.clone();
                        let ticker = ticker.clone();
                        Self::fetch_crypto_analysis(
                            &symbol_for_four_hour,
                            "1d",
                            Rc::new(move |daily_result| {
                                let (daily_analysis, daily_history) = match daily_result {
                                    Ok(result) => result,
                                    Err(message) => {
                                        daily_output.set(error_json("analysis_1d", &message));
                                        return;
                                    }
                                };

                                let lows = daily_history
                                    .candles
                                    .iter()
                                    .map(|candle| candle.low)
                                    .filter(|value| value.is_finite())
                                    .collect::<Vec<_>>();
                                let highs = daily_history
                                    .candles
                                    .iter()
                                    .map(|candle| candle.high)
                                    .filter(|value| value.is_finite())
                                    .collect::<Vec<_>>();
                                let timestamps = daily_history
                                    .candles
                                    .iter()
                                    .filter_map(|candle| candle.timestamp.parse::<u64>().ok())
                                    .collect::<Vec<_>>();
                                let low = lows.iter().copied().reduce(f64::min);
                                let high = highs.iter().copied().reduce(f64::max);
                                let start = timestamps.iter().copied().min();
                                let end = timestamps.iter().copied().max();

                                let payload = json!({
                                    "schemaVersion": "1.0",
                                    "meta": {
                                        "assetType": "crypto",
                                        "symbol": symbol_for_daily,
                                        "exchange": "MEXC Futures",
                                        "source": "MEXC Futures REST",
                                        "timezone": "UTC",
                                        "retrievedAt": browser::now_iso8601()
                                    },
                                    "price": ticker,
                                    "historicalRange": {
                                        "basis": "available_1d_candles",
                                        "candleCount": daily_history.candles.len(),
                                        "startTimestamp": start,
                                        "endTimestamp": end,
                                        "low": low,
                                        "high": high,
                                        "currency": "USDT"
                                    },
                                    "analysis": {
                                        "4h": four_hour_analysis,
                                        "1d": daily_analysis
                                    }
                                });
                                daily_output.set(pretty_json(&payload));
                            }),
                        );
                    }),
                );
            }),
        );
    }

    pub fn load_stock(&self, requested_symbol: &str) {
        let symbol = match normalize_stock_symbol(requested_symbol) {
            Ok(symbol) => symbol,
            Err(message) => {
                self.set_error("validation", message);
                return;
            }
        };

        let output = self.output;
        MarketService::new(MarketApi).load(Rc::new(move |result| {
            let snapshot = match result {
                Ok(response) => response
                    .data
                    .into_iter()
                    .find(|stock| stock.symbol.eq_ignore_ascii_case(&symbol)),
                Err(message) => {
                    output.set(error_json("snapshot", &message));
                    return;
                }
            };

            let Some(snapshot) = snapshot else {
                output.set(error_json(
                    "snapshot",
                    &format!("CafeF snapshot does not contain symbol {symbol}"),
                ));
                return;
            };

            let history_output = output;
            let history_symbol = symbol.clone();
            MarketService::new(MarketApi).fetch_history_raw(
                &symbol,
                Rc::new(move |history_result| {
                    let raw = match history_result {
                        Ok(raw) => raw,
                        Err(message) => {
                            history_output.set(error_json("history", &message));
                            return;
                        }
                    };

                    let history = match crate::domain::market::parse_price_history_response(
                        &raw,
                        &history_symbol,
                    ) {
                        Ok(history) => history,
                        Err(message) => {
                            history_output.set(error_json("history", &message));
                            return;
                        }
                    };

                    let input = match TechnicalAnalysisService::price_history_input(
                        &raw,
                        &history_symbol,
                    ) {
                        Ok(input) => input,
                        Err(message) => {
                            history_output.set(error_json("analysis_1d", &message));
                            return;
                        }
                    };
                    let config = TechnicalAnalysisService::stock_daily_config_for_candles(
                        input.market_data.candles.len(),
                    );
                    let analysis = match TechnicalAnalysisService::analyze(
                        &input,
                        &config,
                        browser::now_iso8601(),
                    ) {
                        Ok(analysis) => analysis,
                        Err(message) => {
                            history_output.set(error_json("analysis_1d", &message));
                            return;
                        }
                    };

                    let lows = history
                        .candles
                        .iter()
                        .map(|candle| candle.low)
                        .filter(|value| value.is_finite())
                        .collect::<Vec<_>>();
                    let highs = history
                        .candles
                        .iter()
                        .map(|candle| candle.high)
                        .filter(|value| value.is_finite())
                        .collect::<Vec<_>>();
                    let low = lows.iter().copied().reduce(f64::min);
                    let high = highs.iter().copied().reduce(f64::max);
                    let first_timestamp = history.candles.first().map(|candle| candle.timestamp.as_str());
                    let last_timestamp = history.candles.last().map(|candle| candle.timestamp.as_str());

                    let payload = json!({
                        "schemaVersion": "1.0",
                        "meta": {
                            "assetType": "stock",
                            "symbol": history_symbol,
                            "exchange": "Vietnam",
                            "source": "CafeF",
                            "timezone": "Asia/Ho_Chi_Minh",
                            "retrievedAt": browser::now_iso8601()
                        },
                        "price": {
                            "symbol": snapshot.symbol,
                            "name": snapshot.name,
                            "lastPrice": snapshot.price,
                            "change": snapshot.change,
                            "changePercent": snapshot.change_percent,
                            "totalVolume": snapshot.total_volume,
                            "marketCap": snapshot.market_cap,
                            "priceUnit": "thousand VND",
                            "volumeUnit": "shares",
                            "marketCapUnit": "VND"
                        },
                        "historicalRange": {
                            "basis": "available_daily_cafef_history",
                            "candleCount": history.candles.len(),
                            "startDate": first_timestamp,
                            "endDate": last_timestamp,
                            "low": low,
                            "high": high,
                            "priceUnit": "thousand VND"
                        },
                        "analysis": {
                            "4h": {
                                "status": "unsupported_timeframe",
                                "reasonCode": "stock_history_is_daily_only"
                            },
                            "1d": analysis
                        }
                    });
                    history_output.set(pretty_json(&payload));
                }),
            );
        }));
    }

    fn set_error(&self, stage: &str, message: String) {
        self.output.set(error_json(stage, &message));
    }

    fn fetch_crypto_analysis(
        symbol: &str,
        timeframe: &'static str,
        on_result: CryptoAnalysisCallback,
    ) {
        let Some((interval, interval_seconds)) =
            TechnicalAnalysisService::mexc_futures_interval(timeframe)
        else {
            on_result(Err(format!("Unsupported timeframe: {timeframe}")));
            return;
        };

        let end_seconds = (js_sys::Date::now().max(0.0) / 1000.0) as u64;
        let start_seconds = end_seconds.saturating_sub(interval_seconds * 500);
        let target_url = format!(
            "{MEXC_REST_BASE}/kline/{symbol}?interval={interval}&start={start_seconds}&end={end_seconds}"
        );
        let symbol = symbol.to_string();
        ProxyService::new(ProxyApi).fetch_raw(
            &target_url,
            Rc::new(move |result| {
                let parsed = result.and_then(|raw| {
                    let input = TechnicalAnalysisService::mexc_futures_klines_input(
                        &raw,
                        &symbol,
                        timeframe,
                    )?;
                    let history =
                        crate::domain::market::parse_mexc_futures_klines_response(&raw, &symbol)?;
                    let config = TechnicalAnalysisService::crypto_config_for_candles(
                        input.market_data.candles.len(),
                    );
                    let analysis = TechnicalAnalysisService::analyze(
                        &input,
                        &config,
                        browser::now_iso8601(),
                    )?;
                    Ok((analysis, history))
                });
                on_result(parsed);
            }),
        );
    }
}

fn parse_ticker_response(raw: &str) -> Result<Value, String> {
    let response: Value = serde_json::from_str(raw)
        .map_err(|error| format!("Invalid MEXC ticker response: {error}"))?;
    if response.get("success").and_then(Value::as_bool) == Some(false) {
        return Err(response
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("MEXC Futures ticker request failed")
            .to_string());
    }
    let data = response
        .get("data")
        .ok_or_else(|| "MEXC ticker response does not contain data".to_string())?;
    let ticker = data
        .as_array()
        .and_then(|items| items.first())
        .cloned()
        .unwrap_or_else(|| data.clone());
    if ticker.get("lastPrice").is_none() {
        return Err("MEXC ticker response does not contain lastPrice".to_string());
    }
    Ok(ticker)
}

fn normalize_coin_symbol(symbol: &str) -> Result<String, String> {
    let symbol = symbol.trim().to_ascii_uppercase();
    let normalized = if symbol.contains('_') {
        symbol
    } else {
        format!("{symbol}_USDT")
    };
    let mut parts = normalized.split('_');
    let base = parts.next().unwrap_or_default();
    let quote = parts.next().unwrap_or_default();
    if base.is_empty()
        || quote != "USDT"
        || parts.next().is_some()
        || !normalized
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err("Invalid symbol. Use a base symbol such as BTC or a Futures symbol such as BTC_USDT.".to_string());
    }
    Ok(normalized)
}

fn normalize_stock_symbol(symbol: &str) -> Result<String, String> {
    let symbol = symbol.trim().to_ascii_uppercase();
    if symbol.is_empty()
        || symbol.len() > 12
        || !symbol.bytes().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
    {
        return Err("Invalid stock symbol. Use an uppercase ticker such as VNM.".to_string());
    }
    Ok(symbol)
}

fn error_json(stage: &str, message: &str) -> String {
    pretty_json(&json!({
        "schemaVersion": "1.0",
        "status": "error",
        "stage": stage,
        "error": message
    }))
}

fn pretty_json(value: &Value) -> String {
    serde_json::to_string_pretty(value)
        .unwrap_or_else(|_| r#"{"status":"error","error":"JSON serialization failed"}"#.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_base_and_full_futures_symbols() {
        assert_eq!(normalize_coin_symbol("btc").unwrap(), "BTC_USDT");
        assert_eq!(normalize_coin_symbol("BTC_USDT").unwrap(), "BTC_USDT");
        assert_eq!(normalize_coin_symbol("1000pepe_usdt").unwrap(), "1000PEPE_USDT");
        assert!(normalize_coin_symbol("BTC_USDT_EXTRA").is_err());
        assert!(normalize_coin_symbol("BTC/path").is_err());
    }

    #[test]
    fn normalizes_vietnam_stock_symbols() {
        assert_eq!(normalize_stock_symbol("vnm").unwrap(), "VNM");
        assert!(normalize_stock_symbol("VNM.US").is_err());
        assert!(normalize_stock_symbol("VNM/path").is_err());
    }
}
