/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />

import { build, files, version } from '$service-worker';

const worker = self as unknown as ServiceWorkerGlobalScope;
const CACHE_NAME = `baconalgo-v${version}`;
const ASSETS = [...build, ...files];

const API_CACHE = 'baconalgo-api-v1';
const IMAGE_CACHE = 'baconalgo-images-v1';

worker.addEventListener('install', (event) => {
	event.waitUntil(
		caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS)).then(() => {
			worker.skipWaiting();
		})
	);
});

worker.addEventListener('activate', (event) => {
	event.waitUntil(
		caches.keys().then(async (keys) => {
			for (const key of keys) {
				if (key !== CACHE_NAME && key !== API_CACHE && key !== IMAGE_CACHE) {
					await caches.delete(key);
				}
			}
			worker.clients.claim();
		})
	);
});

worker.addEventListener('fetch', (event) => {
	const { request } = event;
	const url = new URL(request.url);

	if (request.method !== 'GET') return;

	if (url.pathname.startsWith('/api/')) {
		event.respondWith(networkFirst(request, API_CACHE));
	} else if (request.destination === 'image') {
		event.respondWith(cacheFirst(request, IMAGE_CACHE));
	} else if (ASSETS.includes(url.pathname)) {
		event.respondWith(cacheFirst(request, CACHE_NAME));
	} else {
		event.respondWith(staleWhileRevalidate(request, CACHE_NAME));
	}
});

async function networkFirst(request: Request, cacheName: string): Promise<Response> {
	try {
		const response = await fetch(request);
		if (response.ok) {
			const cache = await caches.open(cacheName);
			cache.put(request, response.clone());
		}
		return response;
	} catch (error) {
		const cached = await caches.match(request);
		if (cached) return cached;
		throw error;
	}
}

async function cacheFirst(request: Request, cacheName: string): Promise<Response> {
	const cached = await caches.match(request);
	if (cached) return cached;

	try {
		const response = await fetch(request);
		if (response.ok) {
			const cache = await caches.open(cacheName);
			cache.put(request, response.clone());
		}
		return response;
	} catch (error) {
		throw error;
	}
}

async function staleWhileRevalidate(request: Request, cacheName: string): Promise<Response> {
	const cached = await caches.match(request);

	const fetchPromise = fetch(request).then(async (response) => {
		if (response.ok) {
			const responseToCache = response.clone();
			const cache = await caches.open(cacheName);
			await cache.put(request, responseToCache);
		}
		return response;
	});

	return cached || fetchPromise;
}

worker.addEventListener('message', (event) => {
	if (event.data && event.data.type === 'SKIP_WAITING') {
		worker.skipWaiting();
	}
});

worker.addEventListener('push', (event) => {
	const data = event.data?.json() ?? {};
	
	const options: NotificationOptions = {
		body: data.body || 'Nouveau signal BaconAlgo',
		icon: '/icon-192.png',
		badge: '/badge-72.png',
		vibrate: [200, 100, 200],
		tag: data.tag || 'baconalgo-notification',
		data: data.data,
		actions: [
			{
				action: 'view',
				title: 'Voir'
			},
			{
				action: 'close',
				title: 'Fermer'
			}
		]
	};

	event.waitUntil(
		worker.registration.showNotification(data.title || 'BaconAlgo', options)
	);
});

worker.addEventListener('notificationclick', (event) => {
	event.notification.close();

	if (event.action === 'view') {
		event.waitUntil(
			worker.clients.openWindow(event.notification.data?.url || '/dashboard')
		);
	}
});

worker.addEventListener('sync', (event) => {
	if (event.tag === 'sync-trades') {
		event.waitUntil(syncTrades());
	}
});

async function syncTrades() {
	console.log('[BaconAlgo SW] Synchronisation des trades...');
}
