---
name: market-data-playground
description: Explain the implemented Vietnam stock and MEXC Futures market-data features, their browser-side data paths, Rust analysis engine, and trading boundaries. Use when users ask about market data, technical analysis, or the Socket page.
---

# Market-data and technical-analysis playground

Current application routes:

- `/#/tools/market` — Vietnam stock market snapshot, CafeF history, and stock analysis.
- `/#/socket` — MEXC Futures realtime ticker monitor, historical-candle analysis, and separate paper/real trading features.

The application is a Leptos CSR/WASM site. These routes are browser-rendered pages, not server-side REST endpoints. Hash routes are client-side navigation and cannot be called as independent HTTP resources by an external agent.

## Implemented data sources and request paths

### Vietnam stocks

- Market snapshots are fetched directly by the browser from CafeF's desktop or mobile market snapshot endpoint, selected according to the detected device.
- Per-symbol daily price history is fetched directly by the browser from CafeF's `{SYMBOL}_PriceHistory.json` resource.
- The Rust `MarketApi` infrastructure adapter performs these requests; `MarketService` coordinates them.
- The current stock analysis flow parses CafeF history and runs the shared Rust `TechnicalAnalysisService`.
- Stock historical data is daily. Do not claim that the current CafeF integration provides intraday 4-hour candles.

### MEXC Futures

- Realtime public ticker updates on the Socket page use the MEXC Futures WebSocket endpoint: `wss://contract.mexc.com/edge`.
- Historical Futures candles for on-demand analysis are fetched from MEXC's contract K-line REST endpoint: `https://api.mexc.com/api/v1/contract/kline/{symbol}`.
- The browser sends those REST requests through the site's configured generic Cloudflare forward proxy at `https://doanson44-forward-proxy.work-sontd.workers.dev/api/proxy`, using the existing `ProxyApi` and `ProxyService` adapters. This proxy is not a dedicated market-data API and does not define a narrow read-only market-data contract.
- Funding-rate requests also use the configured forward proxy. Do not confuse the proxy endpoint with the MEXC WebSocket endpoint or the upstream MEXC REST host.
- Preserve complete Futures contract symbols, including multiplier symbols such as `1000PEPE_USDT`; do not rewrite or strip the multiplier.

## Technical analysis

The browser application's stock and crypto analysis flows use the same Rust technical-analysis engine through `TechnicalAnalysisService`. The analysis output includes configured moving averages, momentum, volatility, trend-strength and volume indicators, price action, market structure, patterns, divergences, scenarios, and data-quality diagnostics as supported by the engine and the available candle history.

- Indicator periods count candles, not calendar days.
- Recent performance fields labelled 1D/5D/20D/60D are calendar-day windows derived from the selected timeframe. If the selected timeframe cannot support a requested window, that field is unavailable rather than mislabeled.
- Pattern and divergence confidence values are heuristic scores, not probabilities.
- Only describe chart-pattern detectors that are actually implemented; do not claim detection of flags, pennants, or cup-and-handle patterns.
- Results depend on upstream data availability, candle history length, and source timestamps.

## External Market Data Worker status

The repository contains source code for an optional, separate Worker under `workers/market-data-api/`. It defines normalized MEXC Futures and CafeF stock endpoints plus an OpenAPI document, but its README explicitly states that deployment has not been performed by that source change.

- Do not treat the Worker source as a live or verified public API.
- Do not publish or invent a production base URL for it.
- The browser application flows described above do not call this Worker; they use the existing CafeF and MEXC integrations and the configured forward proxy.
- The Worker’s technical-indicator implementation is separate from the browser application's Rust engine. Do not assume output parity.

## Planned JSON pages

The proposed `/#/coin/price?symbol=BTC` and `/#/stock/price?symbol=VNM` pages are not documented here as implemented routes until they are present in the application source. If implemented, they should compose the existing data clients and Rust analysis service rather than depend on the undeployed Worker or invent a new API host.

## Trading and safety boundary

- The Socket page includes paper-trading and optional real-trading controls. Real mode can submit MEXC market orders after the user configures exchange credentials and confirms an order.
- Do not describe the entire Socket page as read-only.
- Public ticker, history, and analysis requests are read-only. They must not expose account, position, order-submission, withdrawal, or API-credential operations.
- Never request, copy, or expose exchange API secrets.
- Market data may be delayed, incomplete, unavailable, or changed by the upstream provider. A displayed ticker is not a guaranteed executable price.
