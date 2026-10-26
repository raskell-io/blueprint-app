# blueprint-app

A web app that ships as static files and works offline: SvelteKit prerenders it, Cloudflare Pages serves it, a
service worker caches it, and its CPU-heavy logic is a Rust core compiled to WebAssembly, running in a Web Worker.
It is the app half of a blueprint; the site half (Zola, an ontology, the agent files) is
[blueprint-site](https://github.com/raskell-io/blueprint-site).

The demo renders the Mandelbrot set twice, once in Rust/WebAssembly and once in JavaScript, and shows both timings.
On this tight floating-point loop a modern JavaScript engine is about as fast as WebAssembly, and the page says so.
The reason to reach for Rust is the rest: one core that is tested natively and also runs on a server or a desktop,
types at every boundary, and integer, memory-heavy or multi-threaded work where JavaScript falls behind.

What is in it:

- **A pure Rust core** (`crates/core`), tested with `cargo test`, with no wasm or JS in it.
- **A C ABI boundary** (`crates/wasm`), no wasm-bindgen: the page reads pixels straight out of linear memory.
  The optimised module is 16 KB, and `scripts/build-wasm.sh` fails the build if it grows past its budget.
- **A Web Worker**, so long renders never block the page, with buffers transferred, not copied.
- **Offline**, with SvelteKit's own service worker: one cache per deploy, assets cache-first, pages network-first.
  No plugin, no Workbox.
- **A strict CSP** with hashes: no `unsafe-inline`, no `unsafe-eval`; only `wasm-unsafe-eval`, which
  `WebAssembly.instantiate` needs.
- **Installable**: a web manifest with regular and maskable icons.
- **Agent context**: `AGENTS.md`, `CLAUDE.md`, and path-scoped rules in `.claude/rules/`.

## Run it

```sh
mise install && npm ci
mise run test     # cargo test + clippy (warnings are errors)
mise run dev      # http://127.0.0.1:5173
mise run build    # build/, ready for any static host
```

The Rust toolchain and the `wasm32-unknown-unknown` target come from `rust-toolchain.toml`. `wasm-opt` (from
Binaryen) is used when it is installed and skipped otherwise.

## Deploy

Cloudflare Pages: Workers & Pages > Create > Pages > Connect to Git. Build command
`curl https://sh.rustup.rs -sSf | sh -s -- -y && . "$HOME/.cargo/env" && npm run build`, output directory `build`.
Pages builds every pull request into a preview URL. Add the custom domain in the Pages project.

Two things that matter for an app like this on Pages:

- `static/_headers` sets `Content-Type: application/wasm` and keeps `service-worker.js` uncached, so a new deploy is
  picked up on the next visit.
- If you add threads (`SharedArrayBuffer`), the page needs `Cross-Origin-Embedder-Policy: require-corp`; set it in
  `_headers` on Pages, or, on a host without custom headers, add it from the service worker.

## Licence

MIT.
