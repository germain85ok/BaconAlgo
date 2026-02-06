import type { BrokerInterface, BrokerCredentials, AccountInfo, Position, Order } from './types';

// Stub implementation for Interactive Brokers
export class InteractiveBrokersBroker implements BrokerInterface {
	name = 'ib';
	private connected = false;

	async connect(credentials: BrokerCredentials): Promise<boolean> {
		// TODO: Implement IB TWS API connection
		console.warn('IB broker not yet implemented');
		return false;
	}

	async disconnect(): Promise<void> {
		this.connected = false;
	}

	isConnected(): boolean {
		return this.connected;
	}

	async getAccountInfo(): Promise<AccountInfo> {
		throw new Error('IB broker not yet implemented');
	}

	async getPositions(): Promise<Position[]> {
		return [];
	}

	async getOrders(): Promise<Order[]> {
		return [];
	}

	async placeOrder(): Promise<Order> {
		throw new Error('IB broker not yet implemented');
	}

	async cancelOrder(): Promise<boolean> {
		return false;
	}

	async cancelAllOrders(): Promise<boolean> {
		return false;
	}

	async closeAllPositions(): Promise<boolean> {
		return false;
	}

	async getQuote(): Promise<{ price: number, bid: number, ask: number }> {
		throw new Error('IB broker not yet implemented');
	}
}
