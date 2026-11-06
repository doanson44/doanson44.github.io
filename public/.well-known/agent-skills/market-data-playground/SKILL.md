---
name: market-data-playground
description: Explain the read-only market-data Socket playground on doanson44.github.io, including its public MEXC market-data sources and limitations. Use when users ask about the Socket page or its market data.
---

# Market-data playground

Open `/#/socket` to view the public market-data playground.

## Data sources

- Public MEXC Futures WebSocket market ticker stream: `wss://contract.mexc.com/edge`.
- Funding-rate snapshots are fetched through the site's configured Cloudflare Worker proxy and cached in browser local storage.

## Boundaries

- This is a read-only market-data interface.
- It does not submit orders, withdraw funds, connect to a user's exchange account, or provide personalized investment advice.
- Market data may be delayed, incomplete, unavailable, or changed by the upstream provider.
- Never infer that a displayed rate or ticker is a guaranteed executable price.
- Never request or expose exchange API credentials.

When a feed disconnects, check the upstream provider and browser network status. Do not invent real-time values when the page has no current feed.
