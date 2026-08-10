---
name: build-deploy
description: "Build, test, serve, or deploy the doanson44.github.io platform. Use when: running trunk serve/dev server, building for production with trunk build, running cargo test or cargo clippy, deploying to GitHub Pages, checking CI pipeline, troubleshooting WASM compilation errors. Note: JS is inline in index.html — no separate JS build step needed."
---

# Build & Deploy Skill — doanson44.github.io

## Quick Commands

| Action | Command |
|---|---|
| Dev server | `trunk serve` |
| Production build | `trunk build --release` |
| Run tests | `cargo test` |
| Check compilation | `cargo check --target wasm32-unknown-unknown` |
| Run clippy | `cargo clippy --target wasm32-unknown-unknown -- -D warnings` |
| Format check | `cargo fmt --check` |
| Format fix | `cargo fmt` |

## CI Pipeline (`.github/workflows/deploy.yml`)

Runs on push to `master`:
1. `cargo fmt --check`
2. `cargo check --target wasm32-unknown-unknown`
3. `cargo test`
4. `cargo clippy --target wasm32-unknown-unknown -- -D warnings`
5. Install Trunk
6. `trunk build --release`
7. Deploy `dist/` to GitHub Pages

## Release Profile (`Cargo.toml`)

```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Single codegen unit for better optimization
strip = true          # Strip symbols
panic = "abort"       # Abort on panic
```

## Troubleshooting

### WASM build fails
- Ensure `wasm32-unknown-unknown` target is installed: `rustup target add wasm32-unknown-unknown`
- Check that all `web-sys` features needed are in `Cargo.toml`

### Trunk not found
- Install: `cargo install trunk`
- Or download binary from GitHub releases

### Leptos compilation errors
- Check Leptos version in `Cargo.toml` — must be 0.7.x
- Ensure `csr` feature is enabled: `leptos = { version = "0.7", features = ["csr"] }`
- CSR-only: no `ssr` or `hydrate` features

### GitHub Pages deployment
- Output goes to `dist/` directory (configured in `Trunk.toml`)
- `public_url = "/"` for user site (`username.github.io`)
- For project site, change to `public_url = "/repo-name/"`
