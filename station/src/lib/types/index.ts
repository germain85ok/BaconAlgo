// TypeScript interfaces matching Rust backend types

export interface Signal {
	id: string;
	symbol: string;
	direction: 'LONG' | 'SHORT';
	confidence: number;
	entry_price: number;
	stop_loss: number;
	take_profit: number;
	timeframe: Timeframe;
	timestamp: number;
	indicators: IndicatorValues;
	neural_score: number;
	risk_reward: number;
	status: 'ACTIVE' | 'FILLED' | 'CANCELLED' | 'EXPIRED';
}

export interface Market {
	symbol: string;
	current_price: number;
	change_24h: number;
	volume_24h: number;
	high_24h: number;
	low_24h: number;
	timestamp: number;
}

export interface Portfolio {
	total_value: number;
	cash_balance: number;
	positions: Position[];
	daily_pnl: number;
	total_pnl: number;
	timestamp: number;
}

export interface Position {
	symbol: string;
	quantity: number;
	entry_price: number;
	current_price: number;
	pnl: number;
	pnl_percent: number;
	side: 'LONG' | 'SHORT';
	opened_at: number;
}

export interface Quote {
	symbol: string;
	bid: number;
	ask: number;
	last: number;
	volume: number;
	timestamp: number;
}

export interface HistoricalBar {
	timestamp: number;
	open: number;
	high: number;
	low: number;
	close: number;
	volume: number;
}

export interface IndicatorValues {
	rsi: number;
	macd: { value: number; signal: number; histogram: number };
	stochastic: { k: number; d: number };
	ma50: number;
	ma200: number;
	ema21: number;
	volume_sma: number;
}

export type Timeframe = '1m' | '5m' | '15m' | '1h' | '4h' | '1D' | '1W';

export interface UserPreferences {
	theme: 'dark' | 'light';
	language: 'fr' | 'en';
	notifications_enabled: boolean;
	sound_enabled: boolean;
	default_timeframe: Timeframe;
	favorite_symbols: string[];
	risk_tolerance: 'LOW' | 'MEDIUM' | 'HIGH';
}

export interface SubscriptionTier {
	tier: 'FREE' | 'STATION' | 'PRO' | 'INSTITUTIONAL';
	features: string[];
	max_signals: number;
	max_timeframes: number;
	has_api_access: boolean;
	has_webhooks: boolean;
}

export interface User {
	id: string;
	email: string;
	username: string;
	subscription: SubscriptionTier;
	created_at: number;
	avatar_url?: string;
}

export interface OrderRequest {
	symbol: string;
	side: 'BUY' | 'SELL';
	quantity: number;
	order_type: 'MARKET' | 'LIMIT' | 'STOP_LOSS';
	limit_price?: number;
	stop_price?: number;
}

export interface OrderResponse {
	order_id: string;
	status: 'PENDING' | 'FILLED' | 'CANCELLED' | 'REJECTED';
	filled_quantity: number;
	average_price: number;
	timestamp: number;
}

export interface BacktestResult {
	symbol: string;
	start_date: number;
	end_date: number;
	total_trades: number;
	winning_trades: number;
	losing_trades: number;
	win_rate: number;
	total_profit: number;
	max_drawdown: number;
	sharpe_ratio: number;
}

export interface Alert {
	id: string;
	symbol: string;
	condition: string;
	trigger_price: number;
	is_active: boolean;
	created_at: number;
}

export interface NewsItem {
	id: string;
	title: string;
	summary: string;
	url: string;
	source: string;
	published_at: number;
	symbols: string[];
	sentiment: 'POSITIVE' | 'NEGATIVE' | 'NEUTRAL';
}

export interface WebSocketMessage {
	type: 'SIGNAL' | 'QUOTE' | 'POSITION_UPDATE' | 'ALERT' | 'HEARTBEAT';
	data: any;
	timestamp: number;
}

export interface ApiError {
	code: string;
	message: string;
	details?: any;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	error?: ApiError;
	timestamp: number;
}
