# doanson44 Market Data API

A separate Cloudflare Worker that exposes normalized, read-only MEXC Futures market data for AI agents and external clients.

This Worker is intentionally separate from the GitHub Pages Leptos CSR application. GitHub Pages cannot host server-side API handlers.

## Current scope

- Public MEXC Futures OHLCV candles.
- Public Futures ticker snapshots.
- Public funding-rate snapshots.
- CafeF Vietnam stock snapshots and daily historical candles.
- Standardized technical indicator snapshots calculated from returned OHLCV candles.
- Data-quality metadata, candle close status, source timestamp, and API version.
- OpenAPI 3.1 document at `/openapi.json`.

No account, position, order, withdrawal, API-key, or API-secret endpoints are implemented. Only GET and OPTIONS are accepted.

## Endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| GET | `/health` | Health check |
| GET | `/openapi.json` | OpenAPI document |
| GET | `/v1/crypto/futures/klines` | Normalized OHLCV |
| GET | `/v1/crypto/futures/analysis` | OHLCV and indicators |
| GET | `/v1/crypto/futures/ticker` | Futures ticker |
| GET | `/v1/crypto/futures/funding-rate` | Funding-rate snapshot |

### Example requests

```http
GET /v1/crypto/futures/analysis?symbol=BTC_USDT&timeframe=4h&limit=500
GET /v1/crypto/futures/klines?symbol=1000PEPE_USDT&timeframe=15m&limit=300
GET /v1/crypto/futures/ticker?symbol=BTC_USDT
GET /v1/crypto/futures/funding-rate?symbol=BTC_USDT
GET /v1/stocks/vietnam/snapshot
GET /v1/stocks/vietnam/analysis?symbol=FPT&limit=500
```

Supported timeframes: `1m`, `5m`, `15m`, `30m`, `1h`, `4h`, `8h`, `1d`, `1w`, `1mo`.

`limit` defaults to 500 and is capped at 2000 to match the documented MEXC Futures K-line maximum per request. Optional `start` and `end` are Unix timestamps in seconds.

## Analysis response

The analysis endpoint currently calculates:

- SMA: 20, 50, 100, 200.
- EMA: 9, 20, 50, 200.
- RSI(14), MACD(12,26,9), smoothed Stochastic(14,3,3).
- ATR(14), ATR percentage, Bollinger Bands(20,2), ADX(14).
- Volume SMA(20), OBV.
- Per-indicator availability flags when history is too short.

The analysis endpoints do not yet calculate BOS, CHoCH, swing structure, support/resistance, candlestick patterns, divergence, or a market regime. These are explicitly marked as unavailable rather than fabricated.

Vietnam stock history is daily-only. CafeF does not provide a verified candle-finality field in the data consumed here, so the API marks the latest candle close status as unknown and excludes a current Vietnam calendar-day candle from indicator calculations. Missing-candle checks are not applied to stock history because exchange holidays and non-trading days are not represented by a fixed calendar interval.

Indicator formulas are implemented in this Worker and are versioned independently from the Rust technical-analysis engine currently used by the browser application. Validate parity before treating the two engines as interchangeable.

## Data semantics

- Candle timestamps are Unix seconds and represent candle open time.
- All timestamps are UTC.
- `isClosed` is derived from candle open time plus the timeframe duration and the Worker clock.
- The latest candle may be incomplete. Consumers must inspect `meta.latestCandle.isClosed`.
- Missing intervals are reported in `dataQuality.missingIntervals`.
- Numeric fields are numbers or null; absent indicator values are not replaced with zero.
- MEXC remains the source of truth for the upstream market data. This API does not guarantee executable prices.

## Local development

Requirements: Node.js 20+.

```powershell
npm install
npm run dev
```

Wrangler local development uses the default localhost port. The local origin is allowed for browser testing.

## Deployment

1. Authenticate Wrangler with a Cloudflare account that owns a Workers subdomain.
2. From this directory, run:

```powershell
npx wrangler login
npm run deploy
```

3. Cloudflare will report the deployed `workers.dev` URL. The exact hostname depends on the account's Workers subdomain and must not be guessed.
4. Smoke-test `/health`, `/openapi.json`, and each data endpoint.
5. The OpenAPI endpoint derives its server URL from the incoming request origin.
6. Update the market-data Agent Skill and AI Catalog with the verified URL only after deployment succeeds.

No Cloudflare credentials or deployment secrets are stored in this repository. Deployment has not been performed by this source change.

## CORS

Browser requests are restricted to `https://doanson44.github.io` and localhost development origins. Requests from non-browser agents without an Origin header are accepted. CORS is not authentication; this API contains public market data only.

Before broad public use, configure Cloudflare rate limiting and monitor upstream usage. The Worker does not implement an in-memory rate limiter because per-isolate memory is not a reliable global quota.

## Upstream documentation

- MEXC Futures API: https://mexcdevelop.github.io/apidocs/contract_v1_en/
- Futures K-line endpoint: `GET /api/v1/contract/kline/{symbol}`
- Futures ticker endpoint: `GET /api/v1/contract/ticker`
- Futures funding-rate endpoint: `GET /api/v1/contract/funding_rate/{symbol}`
