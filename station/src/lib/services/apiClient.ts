/**
 * BaconAlgo 2040 - API Client
 * Communication layer between frontend and Rust backend
 */

import type {
	LiveSignal,
	MarketQuote,
	HistoricalData,
	Portfolio,
	UserPreferences,
	ApiResponse,
	SignalWithMetrics
} from '$lib/types/api';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';
const WS_BASE_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:3000';

/**
 * Generic HTTP request wrapper
 */
async function apiRequest<T>(endpoint: string, options?: RequestInit): Promise<ApiResponse<T>> {
	try {
		const response = await fetch(`${API_BASE_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) {
			return {
				success: false,
				error: `HTTP ${response.status}: ${response.statusText}`
			};
		}

		const data: ApiResponse<T> = await response.json();
		return data;
	} catch (error) {
		return {
			success: false,
			error: error instanceof Error ? error.message : 'Unknown error occurred'
		};
	}
}

/**
 * Market Data API
 */
export const marketApi = {
	/**
	 * Get real-time market quotes
	 */
	async getQuotes(symbols?: string[]): Promise<ApiResponse<MarketQuote[]>> {
		const params = symbols ? `?symbols=${symbols.join(',')}` : '';
		return apiRequest<MarketQuote[]>(`/api/quotes${params}`);
	},

	/**
	 * Get historical data for a symbol
	 */
	async getHistoricalData(
		symbol: string,
		interval: string = '1h',
		limit: number = 100
	): Promise<ApiResponse<HistoricalData>> {
		return apiRequest<HistoricalData>(
			`/api/historical/${symbol}?interval=${interval}&limit=${limit}`
		);
	}
};

/**
 * Portfolio API
 */
export const portfolioApi = {
	/**
	 * Get current portfolio
	 */
	async getPortfolio(): Promise<ApiResponse<Portfolio>> {
		return apiRequest<Portfolio>('/api/portfolio');
	}
};

/**
 * Preferences API
 */
export const preferencesApi = {
	/**
	 * Get user preferences
	 */
	async getPreferences(): Promise<ApiResponse<UserPreferences>> {
		return apiRequest<UserPreferences>('/api/preferences');
	},

	/**
	 * Update user preferences
	 */
	async updatePreferences(prefs: UserPreferences): Promise<ApiResponse<UserPreferences>> {
		return apiRequest<UserPreferences>('/api/preferences', {
			method: 'POST',
			body: JSON.stringify(prefs)
		});
	}
};

/**
 * Signals API
 */
export const signalsApi = {
	/**
	 * Get signal details with metrics
	 */
	async getSignalDetails(signalId: string): Promise<ApiResponse<SignalWithMetrics>> {
		return apiRequest<SignalWithMetrics>(`/api/signals/${signalId}`);
	}
};

/**
 * WebSocket Connection Manager
 */
export class SignalWebSocket {
	private ws: WebSocket | null = null;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000;
	private listeners: ((signal: LiveSignal) => void)[] = [];

	/**
	 * Connect to WebSocket
	 */
	connect(): void {
		try {
			this.ws = new WebSocket(`${WS_BASE_URL}/ws/signals`);

			this.ws.onopen = () => {
				console.log('🔥 WebSocket connected');
				this.reconnectAttempts = 0;
			};

			this.ws.onmessage = (event) => {
				try {
					const signal: LiveSignal = JSON.parse(event.data);
					this.listeners.forEach((listener) => listener(signal));
				} catch (error) {
					console.error('Error parsing signal:', error);
				}
			};

			this.ws.onerror = (error) => {
				console.error('WebSocket error:', error);
			};

			this.ws.onclose = () => {
				console.log('WebSocket closed');
				this.attemptReconnect();
			};
		} catch (error) {
			console.error('Failed to create WebSocket:', error);
			this.attemptReconnect();
		}
	}

	/**
	 * Attempt to reconnect
	 */
	private attemptReconnect(): void {
		if (this.reconnectAttempts < this.maxReconnectAttempts) {
			this.reconnectAttempts++;
			console.log(
				`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`
			);
			setTimeout(() => this.connect(), this.reconnectDelay * this.reconnectAttempts);
		} else {
			console.error('Max reconnection attempts reached');
		}
	}

	/**
	 * Subscribe to signals
	 */
	onSignal(callback: (signal: LiveSignal) => void): () => void {
		this.listeners.push(callback);
		
		// Return unsubscribe function
		return () => {
			this.listeners = this.listeners.filter((l) => l !== callback);
		};
	}

	/**
	 * Disconnect WebSocket
	 */
	disconnect(): void {
		if (this.ws) {
			this.ws.close();
			this.ws = null;
		}
		this.listeners = [];
	}

	/**
	 * Check if connected
	 */
	isConnected(): boolean {
		return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
	}
}

/**
 * Server-Sent Events (SSE) Connection Manager
 */
export class SignalSSE {
	private eventSource: EventSource | null = null;
	private listeners: ((signal: LiveSignal) => void)[] = [];

	/**
	 * Connect to SSE
	 */
	connect(): void {
		try {
			this.eventSource = new EventSource(`${API_BASE_URL}/signals/live`);

			this.eventSource.onopen = () => {
				console.log('🔥 SSE connected');
			};

			this.eventSource.onmessage = (event) => {
				try {
					const signal: LiveSignal = JSON.parse(event.data);
					this.listeners.forEach((listener) => listener(signal));
				} catch (error) {
					console.error('Error parsing signal:', error);
				}
			};

			this.eventSource.onerror = (error) => {
				console.error('SSE error:', error);
				this.disconnect();
			};
		} catch (error) {
			console.error('Failed to create EventSource:', error);
		}
	}

	/**
	 * Subscribe to signals
	 */
	onSignal(callback: (signal: LiveSignal) => void): () => void {
		this.listeners.push(callback);
		
		// Return unsubscribe function
		return () => {
			this.listeners = this.listeners.filter((l) => l !== callback);
		};
	}

	/**
	 * Disconnect SSE
	 */
	disconnect(): void {
		if (this.eventSource) {
			this.eventSource.close();
			this.eventSource = null;
		}
		this.listeners = [];
	}

	/**
	 * Check if connected
	 */
	isConnected(): boolean {
		return this.eventSource !== null && this.eventSource.readyState === EventSource.OPEN;
	}
}

// Export singleton instances for convenience
export const signalWebSocket = new SignalWebSocket();
export const signalSSE = new SignalSSE();
