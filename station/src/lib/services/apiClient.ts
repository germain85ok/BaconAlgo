import type {
	Signal,
	Quote,
	Portfolio,
	UserPreferences,
	HistoricalBar,
	OrderRequest,
	OrderResponse,
	ApiResponse,
	WebSocketMessage,
	Timeframe
} from '$lib/types';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';
const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080';

class ApiClient {
	private ws: WebSocket | null = null;
	private eventSource: EventSource | null = null;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 10;
	private reconnectDelay = 1000;
	private listeners: Map<string, Set<(data: any) => void>> = new Map();
	private useWebSocket = true;

	constructor() {
		if (typeof window !== 'undefined') {
			this.initializeConnection();
		}
	}

	private initializeConnection() {
		if (this.useWebSocket) {
			this.connectWebSocket();
		} else {
			this.connectSSE();
		}
	}

	private connectWebSocket() {
		try {
			this.ws = new WebSocket(`${WS_URL}/ws`);

			this.ws.onopen = () => {
				console.log('[BaconAlgo] WebSocket connecté');
				this.reconnectAttempts = 0;
				this.reconnectDelay = 1000;
			};

			this.ws.onmessage = (event) => {
				try {
					const message: WebSocketMessage = JSON.parse(event.data);
					this.notifyListeners(message.type, message.data);
				} catch (error) {
					console.error('[BaconAlgo] Erreur parsing WebSocket:', error);
				}
			};

			this.ws.onerror = (error) => {
				console.error('[BaconAlgo] Erreur WebSocket:', error);
			};

			this.ws.onclose = () => {
				console.log('[BaconAlgo] WebSocket déconnecté');
				this.handleReconnect();
			};
		} catch (error) {
			console.error('[BaconAlgo] Erreur connexion WebSocket:', error);
			this.useWebSocket = false;
			this.connectSSE();
		}
	}

	private connectSSE() {
		try {
			this.eventSource = new EventSource(`${API_BASE_URL}/api/sse`);

			this.eventSource.onopen = () => {
				console.log('[BaconAlgo] SSE connecté');
				this.reconnectAttempts = 0;
			};

			this.eventSource.onmessage = (event) => {
				try {
					const message: WebSocketMessage = JSON.parse(event.data);
					this.notifyListeners(message.type, message.data);
				} catch (error) {
					console.error('[BaconAlgo] Erreur parsing SSE:', error);
				}
			};

			this.eventSource.onerror = (error) => {
				console.error('[BaconAlgo] Erreur SSE:', error);
				this.eventSource?.close();
				this.handleReconnect();
			};
		} catch (error) {
			console.error('[BaconAlgo] Erreur connexion SSE:', error);
		}
	}

	private handleReconnect() {
		if (this.reconnectAttempts >= this.maxReconnectAttempts) {
			console.error('[BaconAlgo] Nombre maximum de reconnexions atteint');
			return;
		}

		this.reconnectAttempts++;
		const delay = this.reconnectDelay * Math.pow(1.5, this.reconnectAttempts - 1);

		console.log(`[BaconAlgo] Reconnexion dans ${delay}ms (tentative ${this.reconnectAttempts})`);

		setTimeout(() => {
			this.initializeConnection();
		}, delay);
	}

	private notifyListeners(type: string, data: any) {
		const listeners = this.listeners.get(type);
		if (listeners) {
			listeners.forEach((callback) => callback(data));
		}
	}

	public subscribe(event: string, callback: (data: any) => void) {
		if (!this.listeners.has(event)) {
			this.listeners.set(event, new Set());
		}
		this.listeners.get(event)!.add(callback);

		return () => {
			this.listeners.get(event)?.delete(callback);
		};
	}

	public async request<T>(endpoint: string, options?: RequestInit): Promise<ApiResponse<T>> {
		try {
			const token = localStorage.getItem('baconalgo_token');
			const headers: HeadersInit = {
				'Content-Type': 'application/json',
				...(token && { Authorization: `Bearer ${token}` }),
				...options?.headers
			};

			const response = await fetch(`${API_BASE_URL}${endpoint}`, {
				...options,
				headers
			});

			const data = await response.json();

			if (!response.ok) {
				return {
					success: false,
					error: data.error || { code: 'UNKNOWN_ERROR', message: 'Erreur inconnue' },
					timestamp: Date.now()
				};
			}

			return {
				success: true,
				data: data,
				timestamp: Date.now()
			};
		} catch (error) {
			return {
				success: false,
				error: {
					code: 'NETWORK_ERROR',
					message: error instanceof Error ? error.message : 'Erreur réseau'
				},
				timestamp: Date.now()
			};
		}
	}

	// API Methods
	async getQuote(symbol: string): Promise<ApiResponse<Quote>> {
		return this.request<Quote>(`/api/quotes/${symbol}`);
	}

	async getHistorical(
		symbol: string,
		timeframe: Timeframe = '1h',
		limit = 100
	): Promise<ApiResponse<HistoricalBar[]>> {
		return this.request<HistoricalBar[]>(
			`/api/historical/${symbol}?timeframe=${timeframe}&limit=${limit}`
		);
	}

	async getPortfolio(): Promise<ApiResponse<Portfolio>> {
		return this.request<Portfolio>('/api/portfolio');
	}

	async createOrder(order: OrderRequest): Promise<ApiResponse<OrderResponse>> {
		return this.request<OrderResponse>('/api/portfolio/position', {
			method: 'POST',
			body: JSON.stringify(order)
		});
	}

	async getPreferences(): Promise<ApiResponse<UserPreferences>> {
		return this.request<UserPreferences>('/api/preferences');
	}

	async updatePreferences(
		preferences: Partial<UserPreferences>
	): Promise<ApiResponse<UserPreferences>> {
		return this.request<UserPreferences>('/api/preferences', {
			method: 'PUT',
			body: JSON.stringify(preferences)
		});
	}

	async getSignals(limit = 10): Promise<ApiResponse<Signal[]>> {
		return this.request<Signal[]>(`/api/signals?limit=${limit}`);
	}

	public disconnect() {
		if (this.ws) {
			this.ws.close();
			this.ws = null;
		}
		if (this.eventSource) {
			this.eventSource.close();
			this.eventSource = null;
		}
		this.listeners.clear();
	}
}

export const apiClient = new ApiClient();
