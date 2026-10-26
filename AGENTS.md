# AGENTS.md

A Svelte app that ships as static files: SvelteKit prerenders it, Cloudflare Pages serves it, a service worker makes
it work offline, and its CPU-heavy logic is a Rust core compiled to WebAssembly, run in a Web Worker. No server, no
account, no data leaves the browser. This file is the contract for agents and people.

## Layout

| Path | What | Rule |
|---|---|---|
| `crates/core/` | The logic, in plain Rust | No wasm, no JS, no I/O. Tested with `cargo test`. |
| `crates/wasm/` | The WebAssembly boundary | A C ABI over the core and nothing else. Numbers and pointers only. |
| `src/lib/render.worker.js` | Loads the module, runs it off the main thread | The only file that touches `app.wasm`. |
| `src/routes/` | The UI (Svelte 5, runes) | Talks to the worker with `postMessage`, never to the module directly. |
| `src/service-worker.js` | Offline support | Versioned cache per deploy; assets cache-first, pages network-first. |
| `vite.config.js` | SvelteKit 3 config: static adapter, CSP with hashes | No `unsafe-inline`, no `unsafe-eval`. |
| `static/` | Manifest, icons, `_headers`, `robots.txt`, `llms.txt`, `.well-known/` | Hand-written; `static/wasm/app.wasm` is generated. |

## The data types

The types are declared once, in `crates/core` (`View`), and cross the boundary as plain numbers in the order
`crates/wasm/src/lib.rs` documents. When a type changes, change the core, the wasm signature and the worker call in
the same commit, and run `mise run test`.

## Verify before reporting done

```sh
mise run test     # cargo test + clippy, warnings are errors
mise run build    # wasm (size budget 64 KB), then the static app into build/
mise run dev
```

Then look at it in a browser: both renders appear, there are no CSP errors in the console, and a reload with the
network off still works.

## Rules that hold

- Static output only (`@sveltejs/adapter-static`, `prerender = true`). Anything that needs a server is a Worker in its
  own repository, not a SvelteKit server route.
- The CSP stays strict. If something needs an inline script or style, it gets a hash in `vite.config.js`, with a
  comment that says why.
- No third-party requests and no telemetry. Fonts are the system stack.
- Exact versions in `package.json`; the toolchain in `rust-toolchain.toml`. Upgrade on purpose, in its own change.
- Keep `app.wasm` inside its budget (`WASM_BUDGET_BYTES` in `scripts/build-wasm.sh`); a new crate dependency must
  justify its bytes.
- Commit and push only when asked.
