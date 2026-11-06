---
name: developer-tools
description: Guide users through the public Markdown, JSON, JWT, and Base64 utilities on doanson44.github.io. Use when a user asks how to navigate or safely use one of these browser-based developer tools.
---

# Developer tools guide

The site provides browser-based utilities under hash routes.

- Markdown Studio: `/#/tools/markdown`
- JSON utility: `/#/tools/json`
- JWT utility: `/#/tools/jwt`
- Base64 utility: `/#/tools/base64`

## Safe-use guidance

1. Treat all pasted input as potentially sensitive.
2. Prefer local, non-production sample data.
3. Do not paste passwords, private keys, production tokens, or personal data into a public page.
4. A JWT decoder can display claims without proving that a token is authentic. Signature verification requires the correct key and algorithm context.
5. Base64 is an encoding, not encryption.
6. Validate generated or transformed output before using it in production.

The tools run in the browser. Do not claim that input is uploaded to a server or retained by the site unless the current implementation explicitly demonstrates that behavior.
