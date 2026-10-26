import adapter from "@sveltejs/adapter-static";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    sveltekit({
      // Static output only: every route is prerendered to a file, so the app is hosted like the site, with no server.
      adapter: adapter({ fallback: "404.html" }),
      // A strict Content Security Policy as a <meta> tag, with hashes for SvelteKit's own inline bootstrap script, so
      // no 'unsafe-inline' is needed. 'wasm-unsafe-eval' is what WebAssembly.instantiate needs, and only that.
      csp: {
        mode: "hash",
        directives: {
          "default-src": ["none"],
          "script-src": ["self", "wasm-unsafe-eval"],
          "worker-src": ["self"],
          // SvelteKit's route announcer (for screen readers) carries one fixed style attribute; allow exactly that
          // attribute by its hash, so every other inline style stays blocked.
          "style-src": ["self", "unsafe-hashes", "sha256-S8qMpvofolR8Mpjy4kQvEm7m1q8clzU4dfDH0AmvZjo="],
          "img-src": ["self", "data:"],
          "connect-src": ["self"],
          "manifest-src": ["self"],
          "base-uri": ["none"],
          "form-action": ["self"],
        },
      },
    }),
  ],
});
