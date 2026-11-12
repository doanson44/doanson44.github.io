# Agent Readiness

This document records what is implemented for the current GitHub Pages deployment and what requires a separately controlled domain or backend.

## Phase 1 — Static discoverability

Implemented in the static site output:

- `/robots.txt` with explicit rules for OAI-SearchBot, GPTBot, Claude-SearchBot, ClaudeBot, PerplexityBot, Google-Extended, Googlebot, and Bingbot.
- Content Signals preference: `ai-train=no, search=yes, ai-input=yes`.
- `/sitemap.xml`, currently containing the canonical application-shell URL only.
- `/llms.txt` with a concise site map and technical limitations.
- HTML discovery links for the sitemap, Agent Skills index, and AI Catalog.

The app uses hash-based client routing. Hash fragments are not independent HTTP resources, so the sitemap intentionally does not pretend that each hash route is a separately crawlable page.

## Phase 2 — Agent integration

Implemented:

- Agent Skills Discovery index at `/.well-known/agent-skills/index.json`, using discovery schema v0.2.0.
- Three focused skills: `site-overview`, `developer-tools`, and `market-data-playground`.
- SHA-256 digests in the index for integrity verification.
- A read-only WebMCP tool, `get_public_site_capabilities`, registered through `document.modelContext` when supported by the browser.
- An AI Catalog / ARD capability manifest at `/.well-known/ai-catalog.json`, listing the three published Agent Skills with `urn:air` identifiers and 2–5 representative queries each. The manifest follows AI Catalog 1.0 and uses the ARD draft's `representativeQueries` extension.

Not published because the corresponding services do not exist:

- An RFC 9727 API Catalog. The configured Cloudflare forward proxy is a generic HTTP forwarder that accepts multiple HTTP methods, including write methods. It is not a narrow, documented, safe-to-advertise public API contract and has no health endpoint or published OpenAPI contract. The repository also contains a separate `workers/market-data-api/` implementation with an OpenAPI document, but its README records that deployment has not been performed or verified. The browser application does not currently call that Worker. Do not advertise either component as a live public market-data API until a read-only deployment is confirmed, reviewed, and equipped with explicit operations, rate limits, and a health endpoint.
- OAuth/OIDC discovery, OAuth Protected Resource Metadata, and `/auth.md`: no authorization server, protected API, or agent-registration flow exists.
- MCP Server Card: WebMCP is a browser-side integration, not a remotely hosted MCP server or transport endpoint.
- DNS-AID records: no remotely callable MCP/A2A agent endpoint exists, and DNS records for `github.io` are controlled by GitHub.

## Phase 3 — Edge and DNS infrastructure

The repository includes an optional Cloudflare Worker template under `infra/cloudflare/agent-readiness-worker/` for a future deployment behind a custom domain. It adds HTTP discovery and Content-Signal headers and CORS for discovery documents.

It is not active for the current `doanson44.github.io` hostname:

- GitHub controls the `github.io` DNS zone. The repository owner cannot create DNS-AID SVCB/HTTPS records or enable DNSSEC for that zone.
- GitHub Pages does not provide per-response custom HTTP header configuration.
- A Cloudflare Worker cannot be routed in front of `doanson44.github.io` unless the site is moved behind a domain/zone controlled by the owner.
- Cloudflare Markdown for Agents requires an eligible Cloudflare zone and the feature enabled there. Cloudflare currently documents availability for Pro, Business, Enterprise, and SSL for SaaS customers. The Worker template alone does not perform HTML-to-Markdown conversion.

To activate Phase 3, attach a custom domain owned by the site owner to GitHub Pages, move/proxy its DNS through Cloudflare, enable DNSSEC, configure the Worker route for that custom hostname, and enable Markdown for Agents in the Cloudflare zone. Only then should DNS-AID records be created, and only if a real agent endpoint is deployed.

## Validation

After merging, validate the published static files with:

```sh
curl -i https://doanson44.github.io/robots.txt
curl -i https://doanson44.github.io/sitemap.xml
curl -i https://doanson44.github.io/.well-known/agent-skills/index.json
curl -i https://doanson44.github.io/.well-known/ai-catalog.json
curl -i https://doanson44.github.io/.well-known/agent-skills/site-overview/SKILL.md
curl -i https://doanson44.github.io/webmcp.js
```

For each Agent Skill, compare the SHA-256 digest of the fetched response body with the digest in the index. The AI Catalog's `representativeQueries` field follows the ARD proposal and is not a finalized IETF standard. The Worker Link headers and Markdown negotiation can only be verified after the Worker is routed through a custom domain. WebMCP requires a supporting secure-context browser; unsupported browsers must degrade gracefully.
