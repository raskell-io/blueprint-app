<script>
  import { onMount } from "svelte";

  const width = 480, height = 320;
  let view = $state({ centerX: -0.7436, centerY: 0.1318, span: 0.012, maxIter: 1500 });
  let wasmMs = $state(null), jsMs = $state(null), busy = $state(false), online = $state(true);
  let wasmCanvas, jsCanvas, worker;

  onMount(() => {
    worker = new Worker(new URL("../lib/render.worker.js", import.meta.url), { type: "module" });
    worker.onmessage = ({ data }) => {
      wasmCanvas.getContext("2d").putImageData(new ImageData(data.wasmPixels, width, height), 0, 0);
      jsCanvas.getContext("2d").putImageData(new ImageData(data.jsPixels, width, height), 0, 0);
      wasmMs = data.wasmMs; jsMs = data.jsMs; busy = false;
    };
    online = navigator.onLine;
    addEventListener("online", () => (online = true));
    addEventListener("offline", () => (online = false));
    render();
    return () => worker.terminate();
  });

  function render() {
    busy = true;
    worker.postMessage({ view: $state.snapshot(view), width, height });
  }
</script>

<svelte:head><title>Blueprint App</title></svelte:head>

<h1>Rust in the browser, offline</h1>
<p class="lede">
  The same Mandelbrot render, twice: once by a Rust core compiled to WebAssembly, once by the same algorithm in
  JavaScript. Both run in a Web Worker, so the page stays responsive. {online ? "You are online." : "You are offline, and it still works."}
</p>
<p class="note">
  On a tight floating-point loop like this one, a modern JavaScript engine is about as fast as WebAssembly. Rust
  earns its place elsewhere: one tested core that also runs natively, types at every boundary, and integer, memory or
  multi-threaded work where JavaScript falls behind.
</p>

<form onsubmit={(e) => { e.preventDefault(); render(); }}>
  <label>Iterations <input type="number" min="50" max="20000" step="50" bind:value={view.maxIter} /></label>
  <button type="submit" disabled={busy}>{busy ? "Rendering…" : "Render"}</button>
</form>

<div class="pair">
  <figure>
    <canvas bind:this={wasmCanvas} {width} {height} aria-label="Mandelbrot set rendered by WebAssembly"></canvas>
    <figcaption>Rust → WebAssembly <strong>{wasmMs === null ? "…" : wasmMs.toFixed(0) + " ms"}</strong></figcaption>
  </figure>
  <figure>
    <canvas bind:this={jsCanvas} {width} {height} aria-label="Mandelbrot set rendered by JavaScript"></canvas>
    <figcaption>JavaScript <strong>{jsMs === null ? "…" : jsMs.toFixed(0) + " ms"}</strong></figcaption>
  </figure>
</div>

<style>
  h1 { font-size: clamp(1.8rem, 5vw, 2.5rem); line-height: 1.1; margin: 0 0 0.75rem; }
  .lede { color: var(--muted); max-width: 40rem; }
  .note { max-width: 40rem; padding: 0.75rem 1rem; border-left: 3px solid var(--accent); background: color-mix(in srgb, var(--accent) 8%, transparent); font-size: 0.95rem; }
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; margin-block: 1.5rem; }
  input { width: 7rem; padding: 0.4rem; border: 1px solid var(--line); border-radius: 0.4rem; background: var(--paper); color: var(--ink); font: inherit; }
  button { padding: 0.5rem 1.1rem; border: 0; border-radius: 2rem; background: var(--ink); color: var(--paper); font: inherit; cursor: pointer; }
  button:disabled { opacity: 0.6; cursor: progress; }
  .pair { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 18rem), 1fr)); gap: 1rem; }
  figure { margin: 0; }
  canvas { display: block; width: 100%; height: auto; border-radius: 0.6rem; background: #0b1412; }
  figcaption { margin-top: 0.4rem; color: var(--muted); font-size: 0.92rem; }
  figcaption strong { color: var(--ink); font-variant-numeric: tabular-nums; }
</style>
