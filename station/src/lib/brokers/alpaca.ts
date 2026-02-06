import type { BrokerInterface, BrokerCredentials, AccountInfo, Position, Order } from './types';

export class AlpacaBroker implements BrokerInterface {
	name = 'alpaca';
	private credentials: BrokerCredentials | null = null;
	private connected = false;
	private baseUrl = '';

	async connect(credentials: BrokerCredentials): Promise<boolean> {
		this.credentials = credentials;
		this.baseUrl = credentials.isPaper 
			? 'https://paper-api.alpaca.markets'
			: 'https://api.alpaca.markets';

		try {
			// Test connection by fetching account info
			const response = await fetch(`${this.baseUrl}/v2/account`, {
				headers: {
					'APCA-API-KEY-ID': credentials.apiKey,
					'APCA-API-SECRET-KEY': credentials.apiSecret
				}
			});

			this.connected = response.ok;
			return this.connected;
		} catch (error) {
			console.error('Alpaca connection error:', error);
			this.connected = false;
			return false;
		}
	}

	async disconnect(): Promise<void> {
		this.connected = false;
		this.credentials = null;
	}

	isConnected(): boolean {
		return this.connected;
	}

	async getAccountInfo(): Promise<AccountInfo> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/account`, {
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		const data = await response.json();

		return {
			cash: parseFloat(data.cash),
			equity: parseFloat(data.equity),
			buyingPower: parseFloat(data.buying_power),
			dayTradeCount: parseInt(data.daytrade_count),
			portfolioValue: parseFloat(data.portfolio_value),
			unrealizedPnL: parseFloat(data.unrealized_pl || '0'),
			realizedPnL: parseFloat(data.realized_pl || '0')
		};
	}

	async getPositions(): Promise<Position[]> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/positions`, {
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		const data = await response.json();

		return data.map((pos: any) => ({
			symbol: pos.symbol,
			quantity: parseFloat(pos.qty),
			avgPrice: parseFloat(pos.avg_entry_price),
			currentPrice: parseFloat(pos.current_price),
			unrealizedPnL: parseFloat(pos.unrealized_pl),
			unrealizedPnLPercent: parseFloat(pos.unrealized_plpc),
			side: pos.side
		}));
	}

	async getOrders(): Promise<Order[]> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/orders`, {
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		const data = await response.json();

		return data.map((order: any) => ({
			id: order.id,
			symbol: order.symbol,
			quantity: parseFloat(order.qty),
			side: order.side,
			type: order.type,
			limitPrice: order.limit_price ? parseFloat(order.limit_price) : undefined,
			stopPrice: order.stop_price ? parseFloat(order.stop_price) : undefined,
			status: order.status,
			filledAt: order.filled_at ? new Date(order.filled_at) : undefined,
			createdAt: new Date(order.created_at)
		}));
	}

	async placeOrder(
		symbol: string,
		quantity: number,
		side: 'buy' | 'sell',
		type: 'market' | 'limit' | 'stop' | 'stop_limit',
		limitPrice?: number,
		stopPrice?: number
	): Promise<Order> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const body: any = {
			symbol,
			qty: quantity,
			side,
			type,
			time_in_force: 'day'
		};

		if (limitPrice) body.limit_price = limitPrice;
		if (stopPrice) body.stop_price = stopPrice;

		const response = await fetch(`${this.baseUrl}/v2/orders`, {
			method: 'POST',
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});

		const data = await response.json();

		return {
			id: data.id,
			symbol: data.symbol,
			quantity: parseFloat(data.qty),
			side: data.side,
			type: data.type,
			limitPrice: data.limit_price ? parseFloat(data.limit_price) : undefined,
			stopPrice: data.stop_price ? parseFloat(data.stop_price) : undefined,
			status: data.status,
			createdAt: new Date(data.created_at)
		};
	}

	async cancelOrder(orderId: string): Promise<boolean> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/orders/${orderId}`, {
			method: 'DELETE',
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		return response.ok;
	}

	async cancelAllOrders(): Promise<boolean> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/orders`, {
			method: 'DELETE',
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		return response.ok;
	}

	async closeAllPositions(): Promise<boolean> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/positions`, {
			method: 'DELETE',
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		return response.ok;
	}

	async getQuote(symbol: string): Promise<{ price: number, bid: number, ask: number }> {
		if (!this.connected || !this.credentials) {
			throw new Error('Not connected to Alpaca');
		}

		const response = await fetch(`${this.baseUrl}/v2/stocks/${symbol}/quotes/latest`, {
			headers: {
				'APCA-API-KEY-ID': this.credentials.apiKey,
				'APCA-API-SECRET-KEY': this.credentials.apiSecret
			}
		});

		const data = await response.json();
		const quote = data.quote;

		return {
			price: (quote.bp + quote.ap) / 2,
			bid: quote.bp,
			ask: quote.ap
		};
	}
}
