import type { BrokerInterface, BrokerCredentials, AccountInfo, Position, Order } from './types';

// Stub implementation for Questrade
export class QuestradeBroker implements BrokerInterface {
	name = 'questrade';
	private connected = false;

	async connect(credentials: BrokerCredentials): Promise<boolean> {
		// TODO: Implement Questrade API connection
		console.warn('Questrade broker not yet implemented');
		return false;
	}

	async disconnect(): Promise<void> {
		this.connected = false;
	}

	isConnected(): boolean {
		return this.connected;
	}

	async getAccountInfo(): Promise<AccountInfo> {
		throw new Error('Questrade broker not yet implemented');
	}

	async getPositions(): Promise<Position[]> {
		return [];
	}

	async getOrders(): Promise<Order[]> {
		return [];
	}

	async placeOrder(): Promise<Order> {
		throw new Error('Questrade broker not yet implemented');
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
		throw new Error('Questrade broker not yet implemented');
	}
}
