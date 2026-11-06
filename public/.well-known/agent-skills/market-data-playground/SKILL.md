---
name: market-data-playground
description: Explain the Stock and Crypto market-analysis features on doanson44.github.io, including data sources, technical indicators, and the separate paper/real trading boundary. Use when users ask about market analysis or the Socket page.
---

# Market-data and technical-analysis playground

Open `/#/tools/market` for Vietnam stock market data and `/#/socket` for the MEXC Futures playground.

## Data sources

- Stock market snapshots and daily history are provided by CafeF.
- Public MEXC Futures ticker updates use the documented WebSocket endpoint: `wss://contract.mexc.com/edge`.
- Historical Futures candles and monthly contract-price extremes use the MEXC Futures REST endpoint on the current API host: `https://api.mexc.com/api/v1/contract/kline/{symbol}`.
- Funding-rate snapshots use the site's configured Cloudflare Worker proxy.
- The WebSocket endpoint and REST API host are distinct; do not infer the WebSocket URL from the REST API host.
- Futures contract symbols, including multiplier symbols such as `1000PEPE_USDT`, must be preserved exactly.

## Technical analysis

The Stock and Crypto analysis flows use the same Rust domain engine. The engine calculates:

- SMA and EMA moving averages.
- RSI, MACD, and smoothed Stochastic.
- ATR, Bollinger Bands, and ADX-based trend strength.
- Volume averages and OBV.
- Candle anatomy, calendar-day returns, price gaps, and consecutive candle moves.
- Swing structure, BOS, CHoCH, support/resistance, and breakout/retest conditions.
- Candlestick patterns, selected chart patterns, and regular RSI/MACD/OBV divergences.
- Market regime, signals, bullish/bearish/range scenarios, and data-quality diagnostics.

Moving-average and oscillator periods count candles. Recent performance fields labelled 1D/5D/20D/60D are calendar-day windows derived from the selected timeframe. On timeframes larger than a requested window, that field is unavailable rather than mislabeled.

Pattern and divergence confidence values are heuristic scores, not probabilities. Chart-pattern detection is limited to the configured detectors; do not claim detection of flags, pennants, or cup-and-handle patterns.

## Trading boundary

- The Socket page contains separate paper-trading and optional real-trading controls. Real mode can submit MEXC market orders after the user configures exchange credentials and confirms an order.
- Do not describe the entire Socket page as read-only.
- The public market-data/analysis integration is read-only. It must not expose account, position, order-submission, withdrawal, or API-credential operations.
- Never request, copy, or expose exchange API secrets.
- Market data may be delayed, incomplete, unavailable, or changed by the upstream provider. Never present a displayed rate or ticker as a guaranteed executable price.
