---
name: site-overview
description: Explain the public doanson44.github.io platform, its client-side architecture, routes, and operational boundaries. Use when an agent needs to understand the website before navigating or describing its capabilities.
---

# Site overview

`doanson44.github.io` is a public, client-side-first developer utility hub and personal portfolio built with Rust, Leptos CSR/WASM, and Trunk.

## Navigation

- `/` — home
- `/#/tools` — tools directory
- `/#/tools/markdown`, `/#/tools/json`, `/#/tools/jwt`, `/#/tools/base64` — developer utilities
- `/#/games` — browser games
- `/#/cv` — CV pages
- `/#/socket` — public market-data socket playground

The application uses hash-based client routing. The server only serves the static application shell; route fragments are not separate HTTP paths.

## Architecture boundaries

- The browser runs the application. GitHub Pages does not host a Rust API server.
- Domain logic is client-side and should not be described as a remotely callable API.
- The Socket page consumes public third-party market data. It does not place trades or manage exchange accounts.
- Never ask users to disclose API keys, passwords, or other secrets in page content.

## Agent interaction

Use WebMCP only when the browser exposes `document.modelContext`. The published site tool is read-only and returns public capability information. Unsupported browsers should continue to work normally.
