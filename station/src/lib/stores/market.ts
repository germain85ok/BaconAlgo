/**
 * Market Store
 * Manages market data and indicators (Fear/Greed Index, VIX, etc.)
 */

import { writable } from 'svelte/store';

export interface TopMover {
	ticker: string;
	change: number;
	changePercent: number;
	price: number;
	volume: number;
}

export interface MarketStatus {
	isOpen: boolean;
	nextOpen: Date | null;
	nextClose: Date | null;
}

// Fear & Greed Index (0-100)
export const fearGreedIndex = writable<number>(50);

// VIX Value
export const vixValue = writable<number>(15.5);

// Top movers
export const topMovers = writable<TopMover[]>([]);

// Market status
export const marketStatus = writable<MarketStatus>({
	isOpen: false,
	nextOpen: null,
	nextClose: null
});

/**
 * Update Fear & Greed Index
 */
export function updateFearGreed(value: number) {
	fearGreedIndex.set(Math.max(0, Math.min(100, value)));
}

/**
 * Update VIX value
 */
export function updateVix(value: number) {
	vixValue.set(Math.max(0, value));
}

/**
 * Update top movers
 */
export function updateTopMovers(movers: TopMover[]) {
	topMovers.set(movers);
}

/**
 * Update market status
 */
export function updateMarketStatus(status: MarketStatus) {
	marketStatus.set(status);
}

/**
 * Check if market is currently open
 */
export function isMarketOpen(): boolean {
	const now = new Date();
	const day = now.getDay(); // 0 = Sunday, 6 = Saturday
	const hour = now.getHours();
	const minute = now.getMinutes();
	const totalMinutes = hour * 60 + minute;

	// Market closed on weekends
	if (day === 0 || day === 6) return false;

	// Market hours: 9:30 AM - 4:00 PM EST
	const marketOpen = 9 * 60 + 30; // 9:30 AM
	const marketClose = 16 * 60; // 4:00 PM

	return totalMinutes >= marketOpen && totalMinutes < marketClose;
}

/**
 * Get next market open time
 */
export function getNextMarketOpen(): Date {
	const now = new Date();
	const next = new Date(now);

	// Set to 9:30 AM
	next.setHours(9, 30, 0, 0);

	// If it's after 4 PM today or weekend, move to next weekday
	if (now.getHours() >= 16 || now.getDay() === 0 || now.getDay() === 6) {
		next.setDate(next.getDate() + 1);
	}

	// Skip weekends
	while (next.getDay() === 0 || next.getDay() === 6) {
		next.setDate(next.getDate() + 1);
	}

	return next;
}

/**
 * Initialize market store with mock data for development
 */
export function initMarketStore() {
	// Set initial mock data
	fearGreedIndex.set(62); // Slightly greedy
	vixValue.set(18.3);

	// Mock top movers
	topMovers.set([
		{ ticker: 'AAPL', change: 2.5, changePercent: 1.4, price: 182.5, volume: 52000000 },
		{ ticker: 'TSLA', change: -3.2, changePercent: -1.2, price: 262.8, volume: 98000000 },
		{ ticker: 'NVDA', change: 8.9, changePercent: 1.8, price: 505.3, volume: 45000000 },
		{ ticker: 'AMD', change: -1.5, changePercent: -0.9, price: 162.1, volume: 38000000 },
		{ ticker: 'MSFT', change: 3.1, changePercent: 0.8, price: 398.2, volume: 28000000 }
	]);

	// Update market status
	const isOpen = isMarketOpen();
	const nextOpen = getNextMarketOpen();

	marketStatus.set({
		isOpen,
		nextOpen: isOpen ? null : nextOpen,
		nextClose: isOpen ? new Date(new Date().setHours(16, 0, 0, 0)) : null
	});

	console.log('Market store initialized');
}

// Auto-update market status every minute
if (typeof window !== 'undefined') {
	setInterval(() => {
		const isOpen = isMarketOpen();
		const nextOpen = getNextMarketOpen();

		marketStatus.set({
			isOpen,
			nextOpen: isOpen ? null : nextOpen,
			nextClose: isOpen ? new Date(new Date().setHours(16, 0, 0, 0)) : null
		});
	}, 60 * 1000);
}
