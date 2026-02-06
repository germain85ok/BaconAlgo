export interface BrokerCredentials {
	apiKey: string;
	apiSecret: string;
	isPaper: boolean;
}

export interface Position {
	symbol: string;
	quantity: number;
	avgPrice: number;
	currentPrice: number;
	unrealizedPnL: number;
	unrealizedPnLPercent: number;
	side: 'long' | 'short';
}

export interface Order {
	id: string;
	symbol: string;
	quantity: number;
	side: 'buy' | 'sell';
	type: 'market' | 'limit' | 'stop' | 'stop_limit';
	limitPrice?: number;
	stopPrice?: number;
	status: 'pending' | 'filled' | 'cancelled' | 'rejected';
	filledAt?: Date;
	createdAt: Date;
}

export interface AccountInfo {
	cash: number;
	equity: number;
	buyingPower: number;
	dayTradeCount: number;
	portfolioValue: number;
	unrealizedPnL: number;
	realizedPnL: number;
}

export interface BrokerInterface {
	name: string;
	connect(credentials: BrokerCredentials): Promise<boolean>;
	disconnect(): Promise<void>;
	isConnected(): boolean;
	
	getAccountInfo(): Promise<AccountInfo>;
	getPositions(): Promise<Position[]>;
	getOrders(): Promise<Order[]>;
	
	placeOrder(
		symbol: string,
		quantity: number,
		side: 'buy' | 'sell',
		type: 'market' | 'limit' | 'stop' | 'stop_limit',
		limitPrice?: number,
		stopPrice?: number
	): Promise<Order>;
	
	cancelOrder(orderId: string): Promise<boolean>;
	cancelAllOrders(): Promise<boolean>;
	closeAllPositions(): Promise<boolean>;
	
	getQuote(symbol: string): Promise<{ price: number, bid: number, ask: number }>;
}
