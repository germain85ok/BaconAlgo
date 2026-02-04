/**
 * Donations Store
 * Manages donations with real-time updates and alerts
 */

import { writable, derived } from 'svelte/store';
import type { Donation, Donor } from '../supabase/client';
import {
	subscribeToDonations,
	getDonations,
	getTopDonors,
	getDonationGoalProgress
} from '../supabase/client';
import { DAILY_DONATION_GOAL, MIN_DONATION_ALERT } from '../config/env';
import type { RealtimeChannel } from '@supabase/supabase-js';

// Store for all donations
export const donations = writable<Donation[]>([]);

// Store for top donors
export const topDonors = writable<Donor[]>([]);

// Store for today's total
export const todaysTotal = writable<number>(0);

// Store for goal progress
export const goalProgress = writable<number>(0);

// Store for active donation alert
export const donationAlert = writable<Donation | null>(null);

// Derived store for recent donations (last 10)
export const recentDonations = derived(donations, ($donations) => $donations.slice(0, 10));

// Add a new donation to the store
export function addDonation(donation: Donation) {
	donations.update((current) => {
		const updated = [donation, ...current];
		// Keep only last 100 donations
		return updated.slice(0, 100);
	});

	// Update today's total
	if (isToday(donation.created_at)) {
		todaysTotal.update((total) => total + donation.amount);
		goalProgress.update((progress) => {
			const newTotal = Math.min(((todaysTotal as any).value + donation.amount) / DAILY_DONATION_GOAL, 1);
			return newTotal;
		});
	}

	// Show alert if donation is above minimum
	if (donation.amount >= MIN_DONATION_ALERT) {
		showDonationAlert(donation);
	}
}

// Show donation alert
export function showDonationAlert(donation: Donation) {
	donationAlert.set(donation);
}

// Clear donation alert
export function clearDonationAlert() {
	donationAlert.set(null);
}

// Helper to check if date is today
function isToday(dateString: string): boolean {
	const date = new Date(dateString);
	const today = new Date();
	return (
		date.getDate() === today.getDate() &&
		date.getMonth() === today.getMonth() &&
		date.getFullYear() === today.getFullYear()
	);
}

// Real-time subscription management
let donationChannel: RealtimeChannel | null = null;

/**
 * Initialize donations store with initial data and real-time subscription
 */
export async function initDonationsStore() {
	try {
		// Load initial donations
		const initialDonations = await getDonations({ limit: 50 });
		donations.set(initialDonations);

		// Load top donors
		const donors = await getTopDonors(10);
		topDonors.set(donors);

		// Load today's progress
		const progress = await getDonationGoalProgress(DAILY_DONATION_GOAL);
		if (progress) {
			todaysTotal.set(progress.total);
			goalProgress.set(progress.progress_percentage / 100);
		}

		// Subscribe to real-time donation updates
		donationChannel = subscribeToDonations((newDonation) => {
			addDonation(newDonation);
		});

		console.log('Donations store initialized with real-time subscription');
	} catch (error) {
		console.error('Failed to initialize donations store:', error);
	}
}

/**
 * Cleanup subscription
 */
export function cleanupDonationsStore() {
	if (donationChannel) {
		donationChannel.unsubscribe();
		donationChannel = null;
	}
}
