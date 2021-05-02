self.addEventListener('install', function(_) {
    console.log('[ServiceWorker] Install');
});

self.addEventListener('activate', function(_) {
    console.log('[ServiceWorker] Activate');
});

self.addEventListener('fetch', function(_) {});