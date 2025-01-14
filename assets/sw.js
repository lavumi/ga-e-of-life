var cacheName = 'game-of-life-cache';
var filesToCache = [
  './',
  './index.html',
  './game_of_life.js',
  './game_of_life_bg.wasm',
];

/* Start the service worker and cache all the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
