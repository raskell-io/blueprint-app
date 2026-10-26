/// <reference types="@sveltejs/kit" />
// Offline support with SvelteKit's built-in service worker: no plugin, no Workbox.
// - `immutable` (the hashed JS and CSS), `assets` (everything in static/, including /wasm/app.wasm) and `prerendered`
//   (the pages) are cached on install, under a cache named after this deploy's `version`, so a new deploy never
//   mixes with an old one.
// - Those assets are served cache-first: their content is fixed for this version.
// - Pages are network-first, falling back to the cached copy when offline.
import { assets, immutable, prerendered } from "$app/manifest";
import { version } from "$app/env";

const CACHE = `app-${version}`;
// Paths come relative to the base path; this app is served from the root.
const ASSETS = new Set([...immutable, ...assets, ...prerendered].map(({ path }) => (path.startsWith("/") ? path : "/" + path)));

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(CACHE).then((c) => c.addAll([...ASSETS])).then(() => self.skipWaiting()));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)))).then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);
  if (event.request.method !== "GET" || url.origin !== location.origin) return;

  if (ASSETS.has(url.pathname)) {
    event.respondWith(caches.match(url.pathname).then((hit) => hit || fetch(event.request)));
    return;
  }
  event.respondWith(
    (async () => {
      const cache = await caches.open(CACHE);
      try {
        const res = await fetch(event.request);
        if (res.ok) cache.put(event.request, res.clone());
        return res;
      } catch {
        return (await cache.match(event.request)) || (await cache.match("/")) || Response.error();
      }
    })(),
  );
});
