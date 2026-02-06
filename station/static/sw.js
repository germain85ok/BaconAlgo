const CACHE_NAME = 'baconalgo-2040-v1';
const API_CACHE = 'baconalgo-api-v1';
const IMAGE_CACHE = 'baconalgo-images-v1';

// URLs essentielles à mettre en cache
const urlsToCache = [
	'/',
	'/offline.html',
	'/app.css',
	'/dashboard',
	'/pricing',
	'/login',
	'/register'
];

// Install event - cache les ressources essentielles
self.addEventListener('install', event => {
	console.log('[SW] Installing Service Worker...', CACHE_NAME);
	event.waitUntil(
		caches.open(CACHE_NAME).then(cache => {
			console.log('[SW] Caching essential resources');
			return cache.addAll(urlsToCache);
		})
	);
	self.skipWaiting();
});

// Activate event - nettoie les anciens caches
self.addEventListener('activate', event => {
	console.log('[SW] Activating Service Worker...', CACHE_NAME);
	event.waitUntil(
		caches.keys().then(cacheNames => {
			return Promise.all(
				cacheNames.map(cacheName => {
					if (
						cacheName !== CACHE_NAME &&
						cacheName !== API_CACHE &&
						cacheName !== IMAGE_CACHE
					) {
						console.log('[SW] Removing old cache:', cacheName);
						return caches.delete(cacheName);
					}
				})
			);
		})
	);
	return self.clients.claim();
});

// Fetch event - stratégie de cache hybride
self.addEventListener('fetch', event => {
	const { request } = event;
	const url = new URL(request.url);

	// Ne pas intercepter les requêtes WebSocket ou SSE
	if (url.protocol === 'ws:' || url.protocol === 'wss:') {
		return;
	}

	// Stratégie différente selon le type de ressource
	if (request.url.includes('/api/')) {
		// API: Network First, fallback to cache
		event.respondWith(networkFirstStrategy(request, API_CACHE));
	} else if (request.destination === 'image') {
		// Images: Cache First, fallback to network
		event.respondWith(cacheFirstStrategy(request, IMAGE_CACHE));
	} else {
		// Autres ressources: Stale While Revalidate
		event.respondWith(staleWhileRevalidateStrategy(request, CACHE_NAME));
	}
});

// Stratégie Network First (pour les API)
async function networkFirstStrategy(request, cacheName) {
	try {
		const response = await fetch(request);
		// Mettre en cache les réponses valides
		if (response && response.status === 200) {
			const cache = await caches.open(cacheName);
			cache.put(request, response.clone());
		}
		return response;
	} catch (error) {
		// Fallback to cache
		const cachedResponse = await caches.match(request);
		if (cachedResponse) {
			return cachedResponse;
		}
		// Si pas de cache, retourner offline page pour navigation
		if (request.mode === 'navigate') {
			return caches.match('/offline.html');
		}
		throw error;
	}
}

// Stratégie Cache First (pour les images)
async function cacheFirstStrategy(request, cacheName) {
	const cachedResponse = await caches.match(request);
	if (cachedResponse) {
		return cachedResponse;
	}

	try {
		const response = await fetch(request);
		if (response && response.status === 200) {
			const cache = await caches.open(cacheName);
			cache.put(request, response.clone());
		}
		return response;
	} catch (error) {
		// Retourner une image de fallback si disponible
		return new Response('', { status: 404 });
	}
}

// Stratégie Stale While Revalidate (pour les autres ressources)
async function staleWhileRevalidateStrategy(request, cacheName) {
	const cachedResponse = await caches.match(request);

	const fetchPromise = fetch(request).then(response => {
		if (response && response.status === 200) {
			const cache = caches.open(cacheName);
			cache.then(c => c.put(request, response.clone()));
		}
		return response;
	});

	return cachedResponse || fetchPromise.catch(() => {
		if (request.mode === 'navigate') {
			return caches.match('/offline.html');
		}
	});
}

// Push Notifications
self.addEventListener('push', event => {
	console.log('[SW] Push notification received');
	
	const options = {
		icon: '/bacon/icon-192.png',
		badge: '/bacon/badge-96.png',
		vibrate: [200, 100, 200],
		tag: 'baconalgo-notification',
		requireInteraction: true,
		actions: [
			{ action: 'view', title: 'Voir', icon: '/bacon/icon-96.png' },
			{ action: 'close', title: 'Fermer', icon: '/bacon/icon-96.png' }
		]
	};

	let data = { title: 'BaconAlgo 2040', body: 'Nouvelle notification' };
	
	if (event.data) {
		try {
			data = event.data.json();
		} catch (e) {
			data.body = event.data.text();
		}
	}

	event.waitUntil(
		self.registration.showNotification(data.title, {
			body: data.body,
			...options,
			data: data
		})
	);
});

// Notification Click
self.addEventListener('notificationclick', event => {
	console.log('[SW] Notification clicked:', event.action);
	event.notification.close();

	if (event.action === 'view' || !event.action) {
		event.waitUntil(
			clients.openWindow(event.notification.data.url || '/dashboard')
		);
	}
});

// Background Sync (pour les requêtes en attente)
self.addEventListener('sync', event => {
	console.log('[SW] Background sync:', event.tag);
	
	if (event.tag === 'sync-signals') {
		event.waitUntil(syncSignals());
	}
});

async function syncSignals() {
	try {
		// Synchroniser les signaux en arrière-plan
		const response = await fetch('/api/signals');
		if (response.ok) {
			const data = await response.json();
			// Envoyer une notification si nouveaux signaux
			if (data.signals && data.signals.length > 0) {
				self.registration.showNotification('Nouveaux Signaux', {
					body: `${data.signals.length} nouveaux signaux disponibles`,
					icon: '/bacon/icon-192.png'
				});
			}
		}
	} catch (error) {
		console.error('[SW] Sync failed:', error);
	}
}

// Message handler (pour communication avec le client)
self.addEventListener('message', event => {
	console.log('[SW] Message received:', event.data);
	
	if (event.data.type === 'SKIP_WAITING') {
		self.skipWaiting();
	}
	
	if (event.data.type === 'CACHE_URLS') {
		event.waitUntil(
			caches.open(CACHE_NAME).then(cache => {
				return cache.addAll(event.data.urls);
			})
		);
	}
	
	if (event.data.type === 'CLEAR_CACHE') {
		event.waitUntil(
			caches.keys().then(cacheNames => {
				return Promise.all(
					cacheNames.map(cacheName => caches.delete(cacheName))
				);
			})
		);
	}
});

console.log('[SW] BaconAlgo 2040 Service Worker loaded');

