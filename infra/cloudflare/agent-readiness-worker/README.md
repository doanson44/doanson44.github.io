# Cloudflare Agent Readiness Worker

This Worker is an optional edge template. It is not active on the current `doanson44.github.io` hostname.

## Prerequisites

1. Register a custom domain that the site owner controls.
2. Add that domain to a Cloudflare zone and configure GitHub Pages as the origin.
3. Confirm the custom hostname serves the site successfully over HTTPS.
4. Update `ORIGIN` in `src/index.js` if the GitHub Pages origin changes.
5. Configure the Worker route in `wrangler.toml` for the custom hostname.
6. Deploy with Wrangler after authenticating to the correct Cloudflare account.

## What it does

- Adds `Content-Signal: ai-train=no, search=yes, ai-input=yes`.
- Adds `Vary: Accept`.
- Adds HTTP `Link` discovery relations on the home page.
- Enables CORS for Agent Skills discovery documents.

## What it does not do

- It does not create DNS-AID records or enable DNSSEC.
- It does not implement OAuth, MCP, A2A, or a first-party API.
- It does not convert HTML into Markdown. Enable Cloudflare Markdown for Agents / Content Converter for the zone separately, where available.
- It must not be deployed with a route for `github.io`; that DNS zone is controlled by GitHub.

## Deployment

After the custom domain is active and the route is configured:

```sh
npx wrangler deploy
```

Test from an external network:

```sh
curl -I https://YOUR_CUSTOM_DOMAIN/
curl -I https://YOUR_CUSTOM_DOMAIN/.well-known/agent-skills/index.json
curl -H "Accept: text/markdown" -I https://YOUR_CUSTOM_DOMAIN/
```

The final command only confirms Markdown negotiation if Cloudflare's conversion feature is enabled and the response reports `Content-Type: text/markdown`.
