---
name: market-data-playground
description: Explain the implemented CafeF Vietnam stock and MEXC Futures market-data integrations, JSON price pages, Rust analysis engine, and Socket trading boundaries. Use when users ask about market data or technical analysis.
---

# Market-data and technical-analysis playground

Current application routes:

- `/#/tools/market` — Vietnam stock market snapshot, CafeF history, and stock analysis.
- `/#/socket` — MEXC Futures realtime ticker monitor, historical-candle analysis, and separate paper/real trading features.
- `/#/coin/price?symbol=BTC` — JSON view for a MEXC Futures contract. A base symbol such as `BTC` is normalized to `BTC_USDT`; a full contract symbol such as `1000PEPE_USDT` is preserved.
- `/#/stock/price?symbol=VNM` — JSON view for one Vietnam-listed stock.

The price pages render a JSON document in the browser and omit the normal site navigation and footer. They are client-rendered hash routes, not HTTP endpoints returning `Content-Type: application/json`; an external agent cannot fetch the hash route as a standalone REST resource without running the application in a browser.

## Implemented data sources and request paths

### Vietnam stocks

- Current market snapshots are fetched directly by the browser from CafeF's desktop or mobile market snapshot endpoint, selected according to the detected device.
- Per-symbol daily price history is fetched directly by the browser from CafeF's `{SYMBOL}_PriceHistory.json` resource.
- The Rust `MarketApi` infrastructure adapter performs these requests; `MarketService` coordinates them.
- The JSON stock page filters the current CafeF snapshot to the requested symbol, derives the observed historical low/high from the returned daily candles, and runs the shared Rust `TechnicalAnalysisService` for `1d`.
- CafeF history is daily-only. The stock page reports `analysis.4h.status = "unsupported_timeframe"` and does not synthesize intraday candles.
- CafeF prices are expressed in thousand VND, volume in shares, and market capitalization in VND.

### MEXC Futures

- Realtime public ticker updates on the Socket page use the MEXC Futures WebSocket endpoint: `wss://contract.mexc.com/edge`.
- The coin JSON page fetches the current Futures ticker and historical contract K-lines from the MEXC Futures REST API through the existing `ProxyApi` and `ProxyService`.
- Historical candles use the contract K-line endpoint: `/api/v1/contract/kline/{symbol}`. The page requests 500 candles for each of `4h` and `1d`.
- The ticker endpoint is `/api/v1/contract/ticker?symbol={symbol}`. The page includes the returned ticker payload in `price`.
- The 1d candle set supplies `historicalRange`; `candleCount`, start/end timestamps, low and high describe the actual returned sample, not all-time history.
- Preserve complete Futures contract symbols, including multiplier symbols such as `1000PEPE_USDT`; do not rewrite or strip the multiplier.

The configured generic Cloudflare forward proxy is `https://doanson44-forward-proxy.work-sontd.workers.dev/api/proxy`. It is a generic relay, not a dedicated market-data API contract.

## JSON response semantics

Both price pages return `schemaVersion`, `meta`, `price`, `historicalRange`, and `analysis`. Errors are returned as JSON with `status: "error"`, `stage`, and `error`; no natural-language market commentary is generated.

- Coin `analysis.4h` and `analysis.1d` are produced by the browser application's shared Rust `TechnicalAnalysisService`.
- Stock `analysis.1d` uses the same Rust engine; stock `analysis.4h` is explicitly unsupported because the available CafeF source history is daily.
- Historical ranges describe the candles returned by the source and must not be described as guaranteed all-time ranges.
- The JSON pages are data views, not trading signals or executable quotes.

## Technical analysis

The browser application's stock and crypto analysis flows use the same Rust technical-analysis engine through `TechnicalAnalysisService`. The analysis output includes configured moving averages, momentum, volatility, trend-strength and volume indicators, price action, market structure, patterns, divergences, scenarios, and data-quality diagnostics as supported by the engine and available candle history.

- Indicator periods count candles, not calendar days.
- Recent performance fields labelled 1D/5D/20D/60D are calendar-day windows derived from the selected timeframe. If the selected timeframe cannot support a requested window, that field is unavailable rather than mislabeled.
- Pattern and divergence confidence values are heuristic scores, not probabilities.
- Only describe chart-pattern detectors that are actually implemented; do not claim detection of flags, pennants, or cup-and-handle patterns.
- Results depend on upstream data availability, candle history length, and source timestamps.

## External Market Data Worker status

The repository contains source code for an optional, separate Worker under `workers/market-data-api/`. It defines normalized MEXC Futures and CafeF stock endpoints plus an OpenAPI document, but deployment has not been verified.

- Do not treat the Worker source as a live or verified public API.
- Do not publish or invent a production base URL for it.
- The browser application and the two JSON price pages do not call this Worker; they use the existing CafeF and MEXC integrations and the configured forward proxy.
- The Worker's technical-indicator implementation is separate from the browser application's Rust engine. Do not assume output parity.

## Trading and safety boundary

- The Socket page includes paper-trading and optional real-trading controls. Real mode can submit MEXC market orders after the user configures exchange credentials and confirms an order.
- Do not describe the entire Socket page as read-only.
- Public ticker, history, and analysis requests are read-only. They must not expose account, position, order-submission, withdrawal, or API-credential operations.
- Never request, copy, or expose exchange API secrets.
- Market data may be delayed, incomplete, unavailable, or changed by the upstream provider. A displayed ticker is not a guaranteed executable price.
