use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::Url;
use worker::{event, Cors, Context, Env, Fetch, Method, Request, Response};

pub mod domain {
    #[path = "../../../src/domain/futures.rs"]
    pub mod futures;
    #[path = "../../../src/domain/market.rs"]
    pub mod market;
    #[path = "../../../src/domain/technical_analysis.rs"]
    pub mod technical_analysis;
}

#[path = "../../../src/application/services/technical_analysis.rs"]
mod technical_analysis_service;

use domain::futures::{parse_mexc_futures_monthly_klines, FuturesPriceExtremes};
use domain::technical_analysis::{AnalysisConfig, AnalysisInput};
use technical_analysis_service::TechnicalAnalysisService;

const CAFE_F_MARKET_URL: &str =
    "https://cafef.vn/du-lieu/ajax/mobile/smart/ajaxbandothitruong.ashx";
const CAFE_F_PRICE_HISTORY_BASE: &str =
    "https://cafefnew.mediacdn.vn/Images/Uploaded/DuLieuDownload/Liveboard/";
const MEXC_FUTURES_API: &str = "https://api.mexc.com/api/v1/contract";
const MEXC_FUNDING_URL: &str = "https://api.mexc.com/api/v1/contract/funding_rate";
const MAX_ANALYSIS_CANDLES: usize = 2_000;
const MAX_ANALYSIS_BODY_BYTES: usize = 1_048_576;
const WEEK_52_SECONDS: u64 = 364 * 24 * 60 * 60;

#[derive(Debug)]
struct ApiError {
    status: u16,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: 400,
            code: "BAD_REQUEST",
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: 404,
            code: "NOT_FOUND",
            message: message.into(),
        }
    }

    fn payload_too_large(message: impl Into<String>) -> Self {
        Self {
            status: 413,
            code: "PAYLOAD_TOO_LARGE",
            message: message.into(),
        }
    }

    fn unprocessable(message: impl Into<String>) -> Self {
        Self {
            status: 422,
            code: "ANALYSIS_FAILED",
            message: message.into(),
        }
    }

    fn upstream(message: impl Into<String>) -> Self {
        Self {
            status: 502,
            code: "UPSTREAM_ERROR",
            message: message.into(),
        }
    }

    fn rate_limited() -> Self {
        Self {
            status: 429,
            code: "RATE_LIMITED",
            message: "Request limit exceeded. Retry after the rate-limit window.".to_string(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: 500,
            code: "INTERNAL_ERROR",
            message: message.into(),
        }
    }
}

impl From<worker::Error> for ApiError {
    fn from(error: worker::Error) -> Self {
        Self::internal(error.to_string())
    }
}

type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Deserialize)]
struct AnalysisRequest {
    input: AnalysisInput,
    config: AnalysisConfig,
}

#[event(fetch)]
pub async fn main(
    mut request: Request,
    env: Env,
    _context: Context,
) -> worker::Result<Response> {
    let cors = Cors::new()
        .with_origins(["*"])
        .with_methods([
            Method::Get,
            Method::Post,
            Method::Head,
            Method::Options,
        ])
        .with_allowed_headers(["Content-Type", "Accept"])
        .with_exposed_headers(["Link", "Cache-Control"])
        .with_max_age(86_400);

    let response = if request.method() == Method::Options {
        Response::empty()?.with_status(204)
    } else {
        let url = request.url()?;
        match handle_request(&mut request, &env, &url).await {
            Ok(response) => response,
            Err(error) => json_response(
                &json!({
                    "error": {
                        "code": error.code,
                        "message": error.message
                    }
                }),
                error.status,
                "no-store",
            )?,
        }
    };

    let mut response = response.with_cors(&cors)?;
    response
        .headers_mut()
        .set("X-Content-Type-Options", "nosniff")?;
    response
        .headers_mut()
        .set("Referrer-Policy", "no-referrer")?;
    Ok(response)
}

async fn handle_request(
    request: &mut Request,
    env: &Env,
    url: &Url,
) -> ApiResult<Response> {
    let method = request.method();
    let path = url.path();

    if method == Method::Head && (path == "/" || path == "/.well-known/api-catalog") {
        return head_discovery_response(url);
    }

    if method == Method::Get && path == "/" {
        return root_response(url);
    }
    if method == Method::Get && path == "/health" {
        return json_response(
            &json!({
                "status": "ok",
                "service": "doanson44-market-intelligence-api",
                "version": "1.0.0"
            }),
            200,
            "no-store",
        )
        .map_err(Into::into);
    }
    if method == Method::Get && path == "/openapi.json" {
        return openapi_response(url);
    }
    if method == Method::Get && path == "/.well-known/api-catalog" {
        return api_catalog_response(url);
    }

    if path.starts_with("/v1/") && !rate_limit_allowed(request, env).await? {
        return Err(ApiError::rate_limited());
    }

    let segments = path
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    match (method, segments.as_slice()) {
        (Method::Get, ["v1", "stocks", "market"]) => get_stock_market().await,
        (Method::Get, ["v1", "stocks", symbol, "analysis"]) => {
            get_stock_analysis(symbol).await
        }
        (Method::Get, ["v1", "crypto", "market"]) => get_crypto_market().await,
        (Method::Get, ["v1", "crypto", "funding"]) => get_crypto_funding(None).await,
        (Method::Get, ["v1", "crypto", symbol, "quote"]) => {
            get_crypto_quote(symbol).await
        }
        (Method::Get, ["v1", "crypto", symbol, "funding"]) => {
            get_crypto_funding(Some(symbol)).await
        }
        (Method::Get, ["v1", "crypto", symbol, "analysis"]) => {
            let timeframe = url
                .query_pairs()
                .find(|(key, _)| key == "timeframe")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_else(|| "1D".to_string());
            get_crypto_analysis(symbol, &timeframe).await
        }
        (Method::Get, ["v1", "crypto", symbol, "extremes"]) => {
            get_crypto_extremes(symbol).await
        }
        (Method::Post, ["v1", "analysis"]) => post_analysis(request).await,
        _ => Err(ApiError::not_found("The requested API route does not exist.")),
    }
}

async fn rate_limit_allowed(request: &Request, env: &Env) -> ApiResult<bool> {
    let client_ip = request
        .headers()
        .get("CF-Connecting-IP")?
        .unwrap_or_else(|| "unknown".to_string());
    let limiter = env.rate_limiter("MARKET_API_LIMITER")?;
    let outcome = limiter.limit(client_ip).await?;
    Ok(outcome.success)
}

async fn get_stock_market() -> ApiResult<Response> {
    let raw = fetch_text(CAFE_F_MARKET_URL).await?;
    let value = parse_json(&raw, "CafeF market snapshot")?;
    if value
        .get("Success")
        .and_then(Value::as_bool)
        .is_some_and(|success| !success)
    {
        return Err(ApiError::upstream("CafeF reported an unsuccessful market response."));
    }

    json_response(
        &json!({
            "source": "CafeF",
            "data": value
        }),
        200,
        "public, max-age=30, stale-while-revalidate=30",
    )
    .map_err(Into::into)
}

async fn get_stock_analysis(symbol: &str) -> ApiResult<Response> {
    let symbol = validate_stock_symbol(symbol)?;
    let url = format!("{CAFE_F_PRICE_HISTORY_BASE}{symbol}_PriceHistory.json");
    let raw = fetch_text(&url).await?;
    let input = TechnicalAnalysisService::price_history_input(&raw, &symbol)
        .map_err(ApiError::unprocessable)?;
    let config =
        TechnicalAnalysisService::stock_daily_config_for_candles(input.market_data.candles.len());
    let result = TechnicalAnalysisService::analyze(&input, &config, now_iso8601())
        .map_err(ApiError::unprocessable)?;

    json_response(
        &result,
        200,
        "public, max-age=60, stale-while-revalidate=60",
    )
    .map_err(Into::into)
}

async fn get_crypto_market() -> ApiResult<Response> {
    let value = fetch_json(&format!("{MEXC_FUTURES_API}/ticker")).await?;
    ensure_mexc_success(&value)?;
    json_response(
        &json!({
            "source": "MEXC Futures",
            "data": value.get("data").cloned().unwrap_or(Value::Null)
        }),
        200,
        "public, max-age=5, stale-while-revalidate=10",
    )
    .map_err(Into::into)
}

async fn get_crypto_quote(symbol: &str) -> ApiResult<Response> {
    let symbol = validate_crypto_symbol(symbol)?;
    let url = format!("{MEXC_FUTURES_API}/ticker?symbol={symbol}");
    let value = fetch_json(&url).await?;
    ensure_mexc_success(&value)?;
    let data = value.get("data").cloned().unwrap_or(Value::Null);

    json_response(
        &json!({
            "source": "MEXC Futures",
            "symbol": symbol,
            "data": data
        }),
        200,
        "public, max-age=5, stale-while-revalidate=10",
    )
    .map_err(Into::into)
}

async fn get_crypto_funding(symbol: Option<&str>) -> ApiResult<Response> {
    let value = fetch_json(MEXC_FUNDING_URL).await?;
    ensure_mexc_success(&value)?;
    let data = value.get("data").cloned().unwrap_or(Value::Null);

    let response_data = if let Some(symbol) = symbol {
        let symbol = validate_crypto_symbol(symbol)?;
        let records = data
            .as_array()
            .ok_or_else(|| ApiError::upstream("MEXC funding response data is not an array."))?;
        let filtered = records
            .iter()
            .filter(|record| {
                record
                    .get("symbol")
                    .and_then(Value::as_str)
                    .is_some_and(|value| value.eq_ignore_ascii_case(&symbol))
            })
            .cloned()
            .collect::<Vec<_>>();
        if filtered.is_empty() {
            return Err(ApiError::not_found(format!(
                "No funding-rate record was returned for {symbol}."
            )));
        }
        json!({
            "source": "MEXC Futures",
            "symbol": symbol,
            "data": filtered
        })
    } else {
        json!({
            "source": "MEXC Futures",
            "data": data
        })
    };

    json_response(
        &response_data,
        200,
        "public, max-age=15, stale-while-revalidate=15",
    )
    .map_err(Into::into)
}

async fn get_crypto_analysis(symbol: &str, timeframe: &str) -> ApiResult<Response> {
    let symbol = validate_crypto_symbol(symbol)?;
    let (interval, _) = TechnicalAnalysisService::mexc_futures_interval(timeframe)
        .ok_or_else(|| ApiError::bad_request("Unsupported MEXC Futures timeframe."))?;
    let url = format!("{MEXC_FUTURES_API}/kline/{symbol}?interval={interval}");
    let raw = fetch_text(&url).await?;
    let input = TechnicalAnalysisService::mexc_futures_klines_input(
        &raw,
        &symbol,
        &timeframe.trim().to_ascii_uppercase(),
    )
    .map_err(ApiError::unprocessable)?;
    let config =
        TechnicalAnalysisService::crypto_config_for_candles(input.market_data.candles.len());
    let result = TechnicalAnalysisService::analyze(&input, &config, now_iso8601())
        .map_err(ApiError::unprocessable)?;

    json_response(
        &result,
        200,
        "public, max-age=15, stale-while-revalidate=15",
    )
    .map_err(Into::into)
}

async fn get_crypto_extremes(symbol: &str) -> ApiResult<Response> {
    let symbol = validate_crypto_symbol(symbol)?;
    let url = format!("{MEXC_FUTURES_API}/kline/{symbol}?interval=Month1&start=0");
    let raw = fetch_text(&url).await?;
    let candles =
        parse_mexc_futures_monthly_klines(&raw, &symbol).map_err(ApiError::unprocessable)?;
    let now_seconds = (js_sys::Date::now().max(0.0) as u64) / 1_000;
    let cutoff = now_seconds.saturating_sub(WEEK_52_SECONDS);
    let extremes = FuturesPriceExtremes::from_monthly_candles(&candles, cutoff);

    json_response(
        &json!({
            "source": "MEXC Futures",
            "symbol": symbol,
            "week_52": {
                "low": extremes.week_52_low,
                "high": extremes.week_52_high
            },
            "all_time": {
                "low": extremes.historical_low,
                "high": extremes.historical_high
            }
        }),
        200,
        "public, max-age=300, stale-while-revalidate=300",
    )
    .map_err(Into::into)
}

async fn post_analysis(request: &mut Request) -> ApiResult<Response> {
    if let Some(content_length) = request.headers().get("Content-Length")? {
        if content_length
            .parse::<usize>()
            .ok()
            .is_some_and(|length| length > MAX_ANALYSIS_BODY_BYTES)
        {
            return Err(ApiError::payload_too_large("Analysis request exceeds 1 MiB."));
        }
    }

    let body = request.bytes().await?;
    if body.len() > MAX_ANALYSIS_BODY_BYTES {
        return Err(ApiError::payload_too_large("Analysis request exceeds 1 MiB."));
    }

    let payload: AnalysisRequest = serde_json::from_slice(&body)
        .map_err(|error| ApiError::bad_request(format!("Invalid analysis request: {error}")))?;
    if payload.input.market_data.candles.len() > MAX_ANALYSIS_CANDLES
        || payload.config.data_requirements.maximum_candles > MAX_ANALYSIS_CANDLES
    {
        return Err(ApiError::payload_too_large(
            "Analysis is limited to 2,000 candles.",
        ));
    }

    let result = TechnicalAnalysisService::analyze(
        &payload.input,
        &payload.config,
        now_iso8601(),
    )
    .map_err(ApiError::unprocessable)?;

    json_response(&result, 200, "no-store").map_err(Into::into)
}

async fn fetch_text(url: &str) -> ApiResult<String> {
    let parsed_url =
        Url::parse(url).map_err(|error| ApiError::internal(format!("Invalid upstream URL: {error}")))?;
    let mut response = Fetch::Url(parsed_url)
        .send()
        .await
        .map_err(|error| ApiError::upstream(format!("Upstream request failed: {error}")))?;
    let status = response.status_code();
    let body = response
        .text()
        .await
        .map_err(|error| ApiError::upstream(format!("Failed to read upstream response: {error}")))?;
    if !(200..300).contains(&status) {
        return Err(ApiError::upstream(format!(
            "Upstream returned HTTP {status}."
        )));
    }
    Ok(body)
}

async fn fetch_json(url: &str) -> ApiResult<Value> {
    let raw = fetch_text(url).await?;
    parse_json(&raw, "upstream JSON")
}

fn parse_json(raw: &str, source: &str) -> ApiResult<Value> {
    serde_json::from_str(raw)
        .map_err(|error| ApiError::upstream(format!("Invalid {source} response: {error}")))
}

fn ensure_mexc_success(value: &Value) -> ApiResult<()> {
    if value
        .get("success")
        .and_then(Value::as_bool)
        .is_some_and(|success| !success)
    {
        let message = value
            .get("message")
            .or_else(|| value.get("msg"))
            .and_then(Value::as_str)
            .unwrap_or("MEXC returned an unsuccessful response.");
        return Err(ApiError::upstream(message.to_string()));
    }
    Ok(())
}

fn validate_stock_symbol(symbol: &str) -> ApiResult<String> {
    let symbol = symbol.trim().to_ascii_uppercase();
    if symbol.is_empty()
        || symbol.len() > 10
        || !symbol.chars().all(|character| character.is_ascii_alphanumeric())
    {
        return Err(ApiError::bad_request("Invalid Vietnam stock symbol."));
    }
    Ok(symbol)
}

fn validate_crypto_symbol(symbol: &str) -> ApiResult<String> {
    let symbol = symbol.trim().to_ascii_uppercase();
    let Some((base, quote)) = symbol.split_once('_') else {
        return Err(ApiError::bad_request(
            "Futures symbol must use BASE_QUOTE format.",
        ));
    };
    if base.is_empty()
        || quote.is_empty()
        || base.len() > 32
        || quote.len() > 12
        || quote.contains('_')
        || !base.chars().all(|character| character.is_ascii_alphanumeric())
        || !quote.chars().all(|character| character.is_ascii_alphanumeric())
    {
        return Err(ApiError::bad_request("Invalid MEXC Futures contract symbol."));
    }
    Ok(symbol)
}

fn now_iso8601() -> String {
    js_sys::Date::new_0()
        .to_iso_string()
        .as_string()
        .unwrap_or_else(|| "unknown".to_string())
}

fn json_response<T: Serialize>(
    body: &T,
    status: u16,
    cache_control: &str,
) -> worker::Result<Response> {
    let mut response = Response::from_json(body)?.with_status(status);
    response
        .headers_mut()
        .set("Cache-Control", cache_control)?;
    Ok(response)
}

fn root_response(url: &Url) -> ApiResult<Response> {
    let origin = url.origin().ascii_serialization();
    let mut response = json_response(
        &json!({
            "name": "doanson44 Market Intelligence API",
            "version": "1.0.0",
            "description": "Read-only Vietnam stock and MEXC Futures market data with the shared Rust technical-analysis engine.",
            "openapi": format!("{origin}/openapi.json"),
            "api_catalog": format!("{origin}/.well-known/api-catalog"),
            "health": format!("{origin}/health")
        }),
        200,
        "no-store",
    )?;
    set_discovery_links(&mut response, &origin)?;
    Ok(response)
}

fn head_discovery_response(url: &Url) -> ApiResult<Response> {
    let origin = url.origin().ascii_serialization();
    let mut response = Response::empty()?.with_status(204);
    set_discovery_links(&mut response, &origin)?;
    let content_type = if url.path() == "/.well-known/api-catalog" {
        "application/linkset+json; charset=utf-8; profile=\"https://www.rfc-editor.org/info/rfc9727\""
    } else {
        "application/json; charset=utf-8"
    };
    response.headers_mut().set("Content-Type", content_type)?;
    Ok(response)
}

fn set_discovery_links(response: &mut Response, origin: &str) -> worker::Result<()> {
    let links = format!(
        "<{origin}/.well-known/api-catalog>; rel=\"api-catalog\"; type=\"application/linkset+json\", <{origin}/openapi.json>; rel=\"service-desc\"; type=\"application/vnd.oai.openapi+json;version=3.1\""
    );
    response.headers_mut().set("Link", &links)
}

fn openapi_response(url: &Url) -> ApiResult<Response> {
    let origin = url.origin().ascii_serialization();
    let spec = openapi_document(&origin);
    let mut response = json_response(&spec, 200, "public, max-age=300")?;
    set_discovery_links(&mut response, &origin)?;
    Ok(response)
}

fn openapi_document(origin: &str) -> Value {
    let json_object = json!({
        "type": "object",
        "additionalProperties": true
    });
    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "doanson44 Market Intelligence API",
            "version": "1.0.0",
            "description": "Read-only market data and technical analysis. No exchange account, order, withdrawal, or credential operations are exposed."
        },
        "servers": [{ "url": origin }],
        "paths": {
            "/v1/stocks/market": {
                "get": {
                    "operationId": "getVietnamStockMarket",
                    "summary": "Get the Vietnam stock market snapshot",
                    "responses": {
                        "200": {
                            "description": "CafeF market snapshot",
                            "content": { "application/json": { "schema": json_object.clone() } }
                        }
                    }
                }
            },
            "/v1/stocks/{symbol}/analysis": {
                "get": {
                    "operationId": "analyzeVietnamStock",
                    "summary": "Run technical analysis for a Vietnam stock",
                    "parameters": [{
                        "name": "symbol",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string", "pattern": "^[A-Z0-9]{1,10}$" }
                    }],
                    "responses": {
                        "200": {
                            "description": "Technical analysis result",
                            "content": { "application/json": { "schema": { "$ref": "#/components/schemas/AnalysisResult" } } }
                        }
                    }
                }
            },
            "/v1/crypto/market": {
                "get": {
                    "operationId": "getCryptoFuturesMarket",
                    "summary": "Get a public MEXC Futures ticker snapshot",
                    "responses": {
                        "200": {
                            "description": "MEXC Futures ticker snapshot",
                            "content": { "application/json": { "schema": json_object.clone() } }
                        }
                    }
                }
            },
            "/v1/crypto/funding": {
                "get": {
                    "operationId": "getCryptoFundingRates",
                    "summary": "Get MEXC Futures funding-rate records",
                    "responses": {
                        "200": {
                            "description": "Funding-rate records",
                            "content": { "application/json": { "schema": json_object.clone() } }
                        }
                    }
                }
            },
            "/v1/crypto/{symbol}/quote": {
                "get": {
                    "operationId": "getCryptoFuturesQuote",
                    "summary": "Get a MEXC Futures contract quote",
                    "parameters": [{
                        "name": "symbol",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string", "pattern": "^[A-Z0-9]+_[A-Z0-9]+$" }
                    }],
                    "responses": {
                        "200": {
                            "description": "Contract ticker",
                            "content": { "application/json": { "schema": json_object.clone() } }
                        }
                    }
                }
            },
            "/v1/crypto/{symbol}/funding": {
                "get": {
                    "operationId": "getCryptoFuturesFundingRate",
                    "summary": "Get funding rate for one MEXC Futures contract",
                    "parameters": [{
                        "name": "symbol",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string", "pattern": "^[A-Z0-9]+_[A-Z0-9]+$" }
                    }],
                    "responses": {
                        "200": {
                            "description": "Contract funding-rate record",
                            "content": { "application/json": { "schema": json_object.clone() } }
                        }
                    }
                }
            },
            "/v1/crypto/{symbol}/analysis": {
                "get": {
                    "operationId": "analyzeCryptoFuturesContract",
                    "summary": "Run the shared technical-analysis engine on MEXC Futures candles",
                    "parameters": [
                        {
                            "name": "symbol",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string", "pattern": "^[A-Z0-9]+_[A-Z0-9]+$" }
                        },
                        {
                            "name": "timeframe",
                            "in": "query",
                            "required": false,
                            "schema": {
                                "type": "string",
                                "enum": ["1M", "5M", "15M", "30M", "1H", "4H", "8H", "1D", "1W", "1MO"],
                                "default": "1D"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Technical analysis result",
                            "content": { "application/json": { "schema": { "$ref": "#/components/schemas/AnalysisResult" } } }
                        }
                    }
                }
            },
            "/v1/crypto/{symbol}/extremes": {
                "get": {
                    "operationId": "getCryptoFuturesPriceExtremes",
                    "summary": "Get historical and 52-week Futures contract price extremes",
                    "parameters": [{
                        "name": "symbol",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string", "pattern": "^[A-Z0-9]+_[A-Z0-9]+$" }
                    }],
                    "responses": {
                        "200": {
                            "description": "Historical and 52-week contract price extremes",
                            "content": { "application/json": { "schema": json_object.clone() } }
                        }
                    }
                }
            },
            "/v1/analysis": {
                "post": {
                    "operationId": "analyzeMarketData",
                    "summary": "Run the shared technical-analysis engine on supplied OHLCV data",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "input": { "type": "object", "description": "AnalysisInput with asset metadata and normalized OHLCV candles." },
                                        "config": { "type": "object", "description": "AnalysisConfig controlling indicators, price action, structure, patterns, divergence, and data quality." }
                                    },
                                    "required": ["input", "config"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Technical analysis result",
                            "content": { "application/json": { "schema": { "$ref": "#/components/schemas/AnalysisResult" } } }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "AnalysisResult": {
                    "type": "object",
                    "description": "Structured result containing data quality, snapshot, price action, trend, momentum, volatility, volume, market structure, support/resistance, breakout, patterns, divergences, regime, signals, scenarios, key levels, and engine summary.",
                    "additionalProperties": true
                }
            }
        }
    })
}

fn api_catalog_response(url: &Url) -> ApiResult<Response> {
    let origin = url.origin().ascii_serialization();
    let doc_url = "https://github.com/doanson44/doanson44.github.io/blob/master/docs/market-intelligence-api.md";
    let linkset = json!({
        "linkset": [
            {
                "anchor": format!("{origin}/v1/stocks/market"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url }],
                "status": [{ "href": format!("{origin}/health") }]
            },
            {
                "anchor": format!("{origin}/v1/stocks/VNM/analysis"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url }],
                "status": [{ "href": format!("{origin}/health") }]
            },
            {
                "anchor": format!("{origin}/v1/crypto/market"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url }],
                "status": [{ "href": format!("{origin}/health") }]
            },
            {
                "anchor": format!("{origin}/v1/crypto/BTC_USDT/analysis?timeframe=4H"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url }],
                "status": [{ "href": format!("{origin}/health") }]
            },
            {
                "anchor": format!("{origin}/v1/crypto/BTC_USDT/funding"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url }],
                "status": [{ "href": format!("{origin}/health") }]
            },
            {
                "anchor": format!("{origin}/v1/crypto/BTC_USDT/quote"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url, "type": "text/markdown" }],
                "status": [{ "href": format!("{origin}/health"), "type": "application/json" }]
            },
            {
                "anchor": format!("{origin}/v1/crypto/funding"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url, "type": "text/markdown" }],
                "status": [{ "href": format!("{origin}/health"), "type": "application/json" }]
            },
            {
                "anchor": format!("{origin}/v1/crypto/BTC_USDT/extremes"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url, "type": "text/markdown" }],
                "status": [{ "href": format!("{origin}/health"), "type": "application/json" }]
            },
            {
                "anchor": format!("{origin}/v1/analysis"),
                "service-desc": [{
                    "href": format!("{origin}/openapi.json"),
                    "type": "application/vnd.oai.openapi+json;version=3.1"
                }],
                "service-doc": [{ "href": doc_url, "type": "text/markdown" }],
                "status": [{ "href": format!("{origin}/health"), "type": "application/json" }]
            }
        ]
    });
    let mut response = Response::from_json(&linkset)?;
    response
        .headers_mut()
        .set("Content-Type", "application/linkset+json; charset=utf-8; profile=\"https://www.rfc-editor.org/info/rfc9727\"")?;
    response
        .headers_mut()
        .set("Cache-Control", "public, max-age=300")?;
    set_discovery_links(&mut response, &origin)?;
    Ok(response)
}
