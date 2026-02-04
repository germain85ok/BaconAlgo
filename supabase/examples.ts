/**
 * BaconAlgo Supabase Integration - Usage Examples
 * 
 * This file demonstrates how to use the Supabase client in your Svelte components
 */

import {
	initSupabase,
	submitDonation,
	getDonations,
	getTodaysDonations,
	getTopDonors,
	getDonationGoalProgress,
	getSignals,
	subscribeToSignals,
	subscribeToDonations,
	getSponsors,
	getSongRequests,
	submitSongRequest,
	unsubscribeAll,
	type Donation,
	type Signal,
	type Donor
} from '$lib/supabase/client';

// =============================================================================
// INITIALIZATION
// =============================================================================

// Initialize Supabase client when your app starts (e.g., in +layout.svelte)
export function initApp() {
	initSupabase();
}

// =============================================================================
// DONATIONS EXAMPLES
// =============================================================================

// Example 1: Submit a donation
export async function handleDonation() {
	try {
		const donation = await submitDonation({
			donor_name: 'John Doe',
			amount: 10.0,
			message: 'Great stream! Keep up the good work!',
			payment_method: 'paypal',
			transaction_id: 'PAYPAL-TXN-123'
		});
		console.log('Donation submitted:', donation);
		return donation;
	} catch (error) {
		console.error('Failed to submit donation:', error);
		throw error;
	}
}

// Example 2: Get recent donations with pagination
export async function loadRecentDonations(page = 0, pageSize = 20) {
	try {
		const donations = await getDonations({
			limit: pageSize,
			offset: page * pageSize,
			status: 'COMPLETED'
		});
		return donations;
	} catch (error) {
		console.error('Failed to load donations:', error);
		return [];
	}
}

// Example 3: Display today's donations
export async function displayTodaysDonations() {
	try {
		const todayDonations = await getTodaysDonations();
		console.log(`Today's donations: ${todayDonations.length}`);
		return todayDonations;
	} catch (error) {
		console.error('Failed to load today donations:', error);
		return [];
	}
}

// Example 4: Show top donors leaderboard
export async function loadTopDonors(limit = 10) {
	try {
		const topDonors = await getTopDonors(limit);
		console.log('Top donors:', topDonors);
		return topDonors;
	} catch (error) {
		console.error('Failed to load top donors:', error);
		return [];
	}
}

// Example 5: Display donation goal progress
export async function showDonationGoal(goalAmount = 100) {
	try {
		const progress = await getDonationGoalProgress(goalAmount);
		console.log(`Goal Progress: ${progress.progress_percentage}%`);
		console.log(`Total: $${progress.total} / $${progress.goal}`);
		console.log(`Remaining: $${progress.remaining}`);
		return progress;
	} catch (error) {
		console.error('Failed to load goal progress:', error);
		return null;
	}
}

// =============================================================================
// REAL-TIME DONATIONS
// =============================================================================

// Example 6: Subscribe to real-time donation alerts
export function setupDonationAlerts(onNewDonation: (donation: Donation) => void) {
	const channel = subscribeToDonations((donation) => {
		console.log('🎉 New donation received!', donation);
		
		// Show alert/notification
		onNewDonation(donation);
		
		// You could trigger animations, sounds, etc.
		if (donation.tier === 'LEGENDARY') {
			console.log('🏆 LEGENDARY DONATION! 🏆');
		}
	});

	return channel;
}

// =============================================================================
// SIGNALS EXAMPLES
// =============================================================================

// Example 7: Get recent trading signals
export async function loadRecentSignals(limit = 50) {
	try {
		const signals = await getSignals({ limit });
		console.log('Recent signals:', signals);
		return signals;
	} catch (error) {
		console.error('Failed to load signals:', error);
		return [];
	}
}

// Example 8: Get high-grade BUY signals only
export async function loadPremiumBuySignals() {
	try {
		const signals = await getSignals({
			direction: 'BUY',
			grade: 'S',
			limit: 20
		});
		return signals;
	} catch (error) {
		console.error('Failed to load premium signals:', error);
		return [];
	}
}

// Example 9: Subscribe to real-time signals with filtering
export function setupSignalAlerts(onNewSignal: (signal: Signal) => void) {
	// Subscribe to all S-grade signals
	const channel = subscribeToSignals(
		(signal) => {
			console.log('🚨 New S-grade signal!', signal);
			onNewSignal(signal);
		},
		{ grade: 'S' }
	);

	return channel;
}

// =============================================================================
// SPONSORS EXAMPLES
// =============================================================================

// Example 10: Load and display sponsors
export async function loadSponsors() {
	try {
		const sponsors = await getSponsors();
		console.log('Active sponsors:', sponsors);
		return sponsors;
	} catch (error) {
		console.error('Failed to load sponsors:', error);
		return [];
	}
}

// =============================================================================
// SONG REQUESTS EXAMPLES
// =============================================================================

// Example 11: Submit a song request
export async function requestSong(name: string, song: string, artist: string) {
	try {
		const request = await submitSongRequest({
			requester_name: name,
			song_title: song,
			artist: artist
		});
		console.log('Song requested:', request);
		return request;
	} catch (error) {
		console.error('Failed to submit song request:', error);
		throw error;
	}
}

// Example 12: Get pending song requests
export async function loadPendingSongs() {
	try {
		const requests = await getSongRequests({ status: 'pending', limit: 20 });
		return requests;
	} catch (error) {
		console.error('Failed to load song requests:', error);
		return [];
	}
}

// =============================================================================
// SVELTE COMPONENT EXAMPLE
// =============================================================================

/**
 * Example Svelte component using the Supabase client
 * 
 * Usage in a .svelte file:
 * 
 * <script lang="ts">
 *   import { onMount, onDestroy } from 'svelte';
 *   import { 
 *     getTodaysDonations, 
 *     subscribeToDonations, 
 *     unsubscribe 
 *   } from '$lib/supabase/client';
 *   import type { Donation } from '$lib/supabase/client';
 * 
 *   let donations: Donation[] = $state([]);
 *   let channel: any;
 * 
 *   onMount(async () => {
 *     // Load initial data
 *     donations = await getTodaysDonations();
 * 
 *     // Subscribe to real-time updates
 *     channel = subscribeToDonations((newDonation) => {
 *       donations = [newDonation, ...donations];
 *       // Show alert animation
 *       showDonationAlert(newDonation);
 *     });
 *   });
 * 
 *   onDestroy(async () => {
 *     // Clean up subscription
 *     if (channel) {
 *       await unsubscribe(channel);
 *     }
 *   });
 * 
 *   function showDonationAlert(donation: Donation) {
 *     // Your alert logic here
 *     console.log('New donation!', donation);
 *   }
 * </script>
 * 
 * <div class="donations-list">
 *   {#each donations as donation}
 *     <div class="donation-item tier-{donation.tier.toLowerCase()}">
 *       <strong>{donation.donor_name}</strong> donated ${donation.amount}
 *       {#if donation.message}
 *         <p>{donation.message}</p>
 *       {/if}
 *     </div>
 *   {/each}
 * </div>
 */

// =============================================================================
// CLEANUP
// =============================================================================

// Example 13: Cleanup all subscriptions (call when user navigates away)
export async function cleanup() {
	try {
		await unsubscribeAll();
		console.log('All subscriptions cleaned up');
	} catch (error) {
		console.error('Failed to cleanup subscriptions:', error);
	}
}

// =============================================================================
// ERROR HANDLING PATTERN
// =============================================================================

// Example 14: Robust error handling with retry logic
export async function loadDataWithRetry<T>(
	fetchFunction: () => Promise<T>,
	maxRetries = 3
): Promise<T | null> {
	for (let i = 0; i < maxRetries; i++) {
		try {
			return await fetchFunction();
		} catch (error) {
			console.error(`Attempt ${i + 1} failed:`, error);
			if (i === maxRetries - 1) {
				console.error('Max retries reached, giving up');
				return null;
			}
			// Wait before retrying (exponential backoff)
			await new Promise((resolve) => setTimeout(resolve, Math.pow(2, i) * 1000));
		}
	}
	return null;
}

// Usage:
// const donations = await loadDataWithRetry(() => getTodaysDonations());
