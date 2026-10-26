// Runs off the main thread, so a long render never freezes the page. It renders the same view twice: once with the
// Rust core compiled to WebAssembly, once with the same algorithm in plain JavaScript, and reports both timings.

let wasm = null;
async function loadWasm() {
  if (!wasm) {
    const bytes = await (await fetch("/wasm/app.wasm")).arrayBuffer();
    const { instance } = await WebAssembly.instantiate(bytes, {});
    wasm = instance.exports;
  }
  return wasm;
}

// Bands of rows: each call is short, so V8 moves the function to its optimising tier after the first few bands
// instead of running the whole image in its baseline tier (WebAssembly has no on-stack replacement).
const BAND = 8;
function renderWasm(exports, v, width, height) {
  const ptr = exports.buffer(width, height);
  for (let row = 0; row < height; row += BAND) {
    exports.render_view(v.centerX, v.centerY, v.span, v.maxIter, width, height, row, Math.min(BAND, height - row));
  }
  // The pixels live in the module's linear memory; copy them out once, into a buffer we can hand to the page.
  return new Uint8ClampedArray(exports.memory.buffer, ptr, width * height * 4).slice();
}

// The same algorithm as crates/core/src/lib.rs, line for line, so the comparison is fair.
function renderJs(v, width, height) {
  const out = new Uint8ClampedArray(width * height * 4);
  const scale = v.span / width, x0 = v.centerX - v.span / 2, y0 = v.centerY - (scale * height) / 2;
  for (let py = 0; py < height; py++) {
    const cy = y0 + py * scale;
    for (let px = 0; px < width; px++) {
      const cx = x0 + px * scale;
      let x = 0, y = 0, i = 0;
      while (i < v.maxIter && x * x + y * y <= 4) { const xt = x * x - y * y + cx; y = 2 * x * y + cy; x = xt; i++; }
      const o = (py * width + px) * 4;
      if (i >= v.maxIter) { out[o] = 11; out[o + 1] = 20; out[o + 2] = 18; }
      else { const t = Math.sqrt(i / v.maxIter); out[o] = 114 * t; out[o + 1] = 40 + 175 * t; out[o + 2] = 50 + 170 * t; }
      out[o + 3] = 255;
    }
  }
  return out;
}

self.onmessage = async ({ data: { view, width, height } }) => {
  const exports = await loadWasm();
  let t = performance.now();
  const wasmPixels = renderWasm(exports, view, width, height);
  const wasmMs = performance.now() - t;
  t = performance.now();
  const jsPixels = renderJs(view, width, height);
  const jsMs = performance.now() - t;
  self.postMessage({ wasmPixels, wasmMs, jsPixels, jsMs }, [wasmPixels.buffer, jsPixels.buffer]);
};
