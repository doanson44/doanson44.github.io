# Agent Readiness

This document records what is implemented for the current GitHub Pages deployment and what requires a separately controlled domain or backend.

## Phase 1 — Static discoverability

Implemented in the static site output:

- `/robots.txt` with explicit rules for OAI-SearchBot, GPTBot, Claude-SearchBot, ClaudeBot, PerplexityBot, Google-Extended, Googlebot, and Bingbot.
- Content Signals preference: `ai-train=no, search=yes, ai-input=yes`.
- `/sitemap.xml`, currently containing the canonical application-shell URL only.
- `/llms.txt` with a concise site map and technical limitations.
- HTML discovery links for the sitemap and Agent Skills index.

The app uses hash-based client routing. Hash fragments are not independent HTTP resources, so the sitemap intentionally does not pretend that each hash route is a separately crawlable page.

## Phase 2 — Agent integration

Implemented:

- Agent Skills Discovery index at `/.well-known/agent-skills/index.json`, using discovery schema v0.2.0.
- Three focused skills: `site-overview`, `developer-tools`, and `market-data-playground`.
- SHA-256 digests in the index for integrity verification.
- A read-only WebMCP tool, `get_public_site_capabilities`, registered through `document.modelContext` when supported by the browser.

Not published because the corresponding services do not exist:

- OAuth/OIDC authorization-server metadata.
- OAuth protected-resource metadata.
- `/auth.md`.
- MCP Server Card or MCP endpoint.
- A first-party API Catalog. RFC 9727 requires a catalog of actual publisher API endpoints; this static client-side application does not expose a first-party API. The existing third-party market-data proxy is not advertised as a public API because it is not a documented site-owned API contract.
- ARD/AI Catalog entries for MCP or A2A services. Publishing empty or fictitious service entries would be misleading.

## Phase 3 — Edge and DNS infrastructure

The repository includes an optional Cloudflare Worker template under `infra/cloudflare/agent-readiness-worker/` for a future deployment behind a custom domain. It adds HTTP discovery and Content-Signal headers and CORS for discovery documents.

It is not active for the current `doanson44.github.io` hostname:

- GitHub controls the `github.io` DNS zone. The repository owner cannot create DNS-AID SVCB/HTTPS records or enable DNSSEC for that zone.
- GitHub Pages does not provide per-response custom HTTP header configuration.
- A Cloudflare Worker cannot be routed in front of `doanson44.github.io` unless the site is moved behind a domain/zone controlled by the owner.
- Cloudflare Markdown for Agents requires an eligible Cloudflare zone and the feature enabled there. The Worker template alone does not perform HTML-to-Markdown conversion.

To activate Phase 3, attach a custom domain owned by the site owner to GitHub Pages, move/proxy its DNS through Cloudflare, enable DNSSEC, configure the Worker route for that custom hostname, and enable Markdown for Agents in the Cloudflare zone. Only then should DNS-AID records be created, and only if a real agent endpoint is deployed.

## Validation

After merging, validate the published static files with:

```sh
curl -i https://doanson44.github.io/robots.txt
curl -i https://doanson44.github.io/sitemap.xml
curl -i https://doanson44.github.io/.well-known/agent-skills/index.json
curl -i https://doanson44.github.io/.well-known/agent-skills/site-overview/SKILL.md
curl -i https://doanson44.github.io/webmcp.js
```

For each Agent Skill, compare the SHA-256 digest of the fetched response body with the digest in the index. WebMCP requires a supporting secure-context browser; unsupported browsers must degrade gracefully.
