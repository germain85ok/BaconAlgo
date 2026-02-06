/**
 * Shared TypeScript types matching Rust backend API
 * BaconAlgo 2040 - Type Definitions
 */

export interface LiveSignal {
	id: string;
	symbol: string;
	horizon: string;
	ready: boolean;
	tags: SignalTags;
	confidence?: number;
	direction?: 'BUY' | 'SELL';
	timestamp: string;
}

export interface SignalTags {
	near_npoc?: boolean;
	near_avwap?: boolean;
	in_golden_pocket?: boolean;
	structure?: string;
	[key: string]: any;
}

export interface MarketQuote {
	symbol: string;
	price: number;
	change: number;
	change_percent: number;
	volume?: number;
	timestamp: string;
}

export interface HistoricalData {
	symbol: string;
	interval: string;
	data: CandleData[];
}

export interface CandleData {
	timestamp: string;
	open: number;
	high: number;
	low: number;
	close: number;
	volume: number;
}

export interface Position {
	id: string;
	symbol: string;
	quantity: number;
	entry_price: number;
	current_price: number;
	pnl: number;
	pnl_percent: number;
	opened_at: string;
}

export interface Portfolio {
	positions: Position[];
	total_value: number;
	cash_balance: number;
	total_pnl: number;
	total_pnl_percent: number;
}

export interface UserPreferences {
	user_id: string;
	theme: string;
	notifications_enabled: boolean;
	sound_alerts: boolean;
	default_timeframe: string;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	error?: string;
}

export interface SignalWithMetrics {
	signal: LiveSignal;
	timeframes: TimeframeAnalysis[];
	indicators: IndicatorMetrics;
}

export interface TimeframeAnalysis {
	timeframe: string; // 1m, 5m, 15m, 1h, 4h, 1D
	trend: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
	strength: number; // 0-100
}

export interface IndicatorMetrics {
	leading: LeadingIndicators;
	lagging: LaggingIndicators;
}

export interface LeadingIndicators {
	rsi?: number;
	stochastic?: number;
	macd_signal?: string;
}

export interface LaggingIndicators {
	ma_50?: number;
	ma_200?: number;
	ema_21?: number;
}

export type Timeframe = '1m' | '5m' | '15m' | '1h' | '4h' | '1D';
export type SignalDirection = 'BUY' | 'SELL';
export type TrendType = 'BULLISH' | 'BEARISH' | 'NEUTRAL';
