/**
 * Signals Store
 * Manages trading signals with real-time updates from Supabase
 */

import { writable, derived } from 'svelte/store';
import type { Signal, SignalGrade } from '../supabase/client';
import { subscribeToSignals, getSignals } from '../supabase/client';
import type { RealtimeChannel } from '@supabase/supabase-js';

// Store for all signals
export const signals = writable<Signal[]>([]);

// Derived store for hot signals (grade S or A)
export const hotSignals = derived(signals, ($signals) =>
	$signals.filter((s) => s.grade === 'S' || s.grade === 'A').slice(0, 5)
);

// Derived store for buy signals
export const buySignals = derived(signals, ($signals) =>
	$signals.filter((s) => s.direction === 'BUY')
);

// Derived store for sell signals
export const sellSignals = derived(signals, ($signals) =>
	$signals.filter((s) => s.direction === 'SELL')
);

// Add a new signal to the store
export function addSignal(signal: Signal) {
	signals.update((current) => {
		// Add to beginning of array
		const updated = [signal, ...current];
		// Keep only last 100 signals
		return updated.slice(0, 100);
	});
}

// Clear old signals (older than X hours)
export function clearOldSignals(hoursOld: number = 24) {
	const cutoffTime = new Date(Date.now() - hoursOld * 60 * 60 * 1000);

	signals.update((current) =>
		current.filter((signal) => new Date(signal.created_at) > cutoffTime)
	);
}

// Real-time subscription management
let signalChannel: RealtimeChannel | null = null;

/**
 * Initialize signals store with initial data and real-time subscription
 */
export async function initSignalsStore() {
	try {
		// Load initial signals
		const initialSignals = await getSignals({ limit: 50 });
		signals.set(initialSignals);

		// Subscribe to real-time updates
		signalChannel = subscribeToSignals((newSignal) => {
			addSignal(newSignal);
		});

		console.log('Signals store initialized with real-time subscription');
	} catch (error) {
		console.error('Failed to initialize signals store:', error);
	}
}

/**
 * Cleanup subscription
 */
export function cleanupSignalsStore() {
	if (signalChannel) {
		signalChannel.unsubscribe();
		signalChannel = null;
	}
}

// Auto-cleanup old signals every 5 minutes
if (typeof window !== 'undefined') {
	setInterval(() => clearOldSignals(24), 5 * 60 * 1000);
}
