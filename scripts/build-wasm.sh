#!/usr/bin/env bash
# Build the WebAssembly module into static/wasm/app.wasm, optimise it if wasm-opt is installed, and enforce a size
# budget so a dependency cannot quietly double the download.
set -euo pipefail
cd "$(dirname "$0")/.."
BUDGET=${WASM_BUDGET_BYTES:-65536}
cargo build --release --target wasm32-unknown-unknown -p app-wasm
in=target/wasm32-unknown-unknown/release/app_wasm.wasm
out=static/wasm/app.wasm
mkdir -p static/wasm
if command -v wasm-opt >/dev/null; then wasm-opt -O3 --enable-bulk-memory --enable-nontrapping-float-to-int "$in" -o "$out"; else cp "$in" "$out"; fi
size=$(wc -c < "$out" | tr -d ' ')
echo "app.wasm: $size bytes (budget $BUDGET)"
[ "$size" -le "$BUDGET" ] || { echo "over the size budget"; exit 1; }
