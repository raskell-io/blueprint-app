---
paths: src/**
---

# Svelte rules

- Svelte 5 runes (`$state`, `$derived`, `$props`); no stores for local state.
- The UI never calls WebAssembly directly. Heavy work goes through `src/lib/render.worker.js` so the main thread
  stays free; the worker transfers buffers instead of copying them.
- No inline `style=` attributes and no `{@html}` with untrusted input: the CSP and the reader both lose.
- Accessibility: one `h1`, labelled inputs, `aria-label` on canvases and icon buttons, visible focus.
- Every route is prerendered; nothing may depend on a server at request time.
