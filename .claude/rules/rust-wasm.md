---
paths: crates/**/*.rs
---

# Rust and WebAssembly rules

- Logic goes in `crates/core`; `crates/wasm` only converts arguments and calls into it. If a change to `crates/wasm`
  is more than a few lines, the logic belongs in the core.
- The core stays target-independent: no `wasm_bindgen`, no `web_sys`, no `std::fs`/`std::net`. That keeps it
  testable with plain `cargo test` and reusable natively.
- Long computations are exported so the caller can split them (bands, steps): V8 starts WebAssembly in a baseline
  tier and has no on-stack replacement, so one long call never gets the optimised code.
- No panics across the boundary in production paths: validate inputs in the wasm crate and return a status.
- `cargo clippy -- -D warnings` must pass. Every exported function documents its arguments and its safety.
