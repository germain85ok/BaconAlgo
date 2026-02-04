/**
 * Supabase Client Configuration
 * Provides database access and real-time subscriptions for BaconAlgo
 */

import { createClient, SupabaseClient, RealtimeChannel } from '@supabase/supabase-js';
import { SUPABASE_URL, SUPABASE_ANON_KEY } from '../config/env';

// Types
export type SignalDirection = 'BUY' | 'SELL';
export type SignalGrade = 'S' | 'A' | 'B' | 'C';
export type DonationStatus = 'PENDING' | 'COMPLETED' | 'FAILED';
export type DonationTier = 'BACON_BIT' | 'CRISPY' | 'SIZZLING' | 'LEGENDARY';

export interface Signal {
	id: string;
	ticker: string;
	direction: SignalDirection;
	grade: SignalGrade;
	score: number;
	indicators: Record<string, any>;
	created_at: string;
}

export interface Donation {
	id: string;
	donor_name: string;
	amount: number;
	message: string | null;
	tier: DonationTier;
	status: DonationStatus;
	payment_method: string;
	transaction_id: string | null;
	created_at: string;
}

export interface Donor {
	id: string;
	name: string;
	total_donated: number;
	donation_count: number;
	first_donation_at: string;
	last_donation_at: string;
}

export interface Sponsor {
	id: string;
	name: string;
	logo_url: string;
	website_url: string;
	active: boolean;
	display_order: number;
	created_at: string;
}

export interface SongRequest {
	id: string;
	requester_name: string;
	song_title: string;
	artist: string;
	status: string;
	created_at: string;
}

export interface DonationGoalProgress {
	total: number;
	goal: number;
	progress_percentage: number;
	remaining: number;
}

// Initialize Supabase client
let supabase: SupabaseClient;

export function initSupabase(): SupabaseClient {
	if (!supabase) {
		if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
			throw new Error('Supabase URL and Anon Key are required');
		}
		supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
			auth: {
				persistSession: false
			},
			realtime: {
				params: {
					eventsPerSecond: 10
				}
			}
		});
	}
	return supabase;
}

// Get or initialize client
export function getSupabaseClient(): SupabaseClient {
	if (!supabase) {
		return initSupabase();
	}
	return supabase;
}

// =============================================================================
// DONATIONS API
// =============================================================================

/**
 * Submit a new donation
 */
export async function submitDonation(donation: {
	donor_name: string;
	amount: number;
	message?: string;
	payment_method: string;
	transaction_id?: string;
}) {
	const client = getSupabaseClient();

	// Determine tier based on amount
	let tier: DonationTier = 'BACON_BIT';
	if (donation.amount >= 20) tier = 'LEGENDARY';
	else if (donation.amount >= 10) tier = 'SIZZLING';
	else if (donation.amount >= 5) tier = 'CRISPY';

	const { data, error } = await client
		.from('donations')
		.insert({
			donor_name: donation.donor_name,
			amount: donation.amount,
			message: donation.message || null,
			tier,
			status: 'PENDING',
			payment_method: donation.payment_method,
			transaction_id: donation.transaction_id || null
		})
		.select()
		.single();

	if (error) throw error;
	return data as Donation;
}

/**
 * Get all donations with pagination
 */
export async function getDonations(options?: {
	limit?: number;
	offset?: number;
	status?: DonationStatus;
}) {
	const client = getSupabaseClient();
	const { limit = 50, offset = 0, status } = options || {};

	let query = client
		.from('donations')
		.select('*')
		.order('created_at', { ascending: false })
		.range(offset, offset + limit - 1);

	if (status) {
		query = query.eq('status', status);
	}

	const { data, error } = await query;
	if (error) throw error;
	return data as Donation[];
}

/**
 * Get today's donations
 */
export async function getTodaysDonations() {
	const client = getSupabaseClient();
	const { data, error } = await client.rpc('get_todays_donations');
	if (error) throw error;
	return data as Donation[];
}

/**
 * Get top donors
 */
export async function getTopDonors(limit: number = 10) {
	const client = getSupabaseClient();
	const { data, error } = await client.rpc('get_top_donors', { limit_count: limit });
	if (error) throw error;
	return data as Donor[];
}

/**
 * Get donation goal progress
 */
export async function getDonationGoalProgress(goal: number) {
	const client = getSupabaseClient();
	const { data, error } = await client.rpc('get_donation_goal_progress', { goal_amount: goal });
	if (error) throw error;
	return data as DonationGoalProgress;
}

// =============================================================================
// SIGNALS API
// =============================================================================

/**
 * Get trading signals
 */
export async function getSignals(options?: {
	limit?: number;
	offset?: number;
	direction?: SignalDirection;
	grade?: SignalGrade;
}) {
	const client = getSupabaseClient();
	const { limit = 50, offset = 0, direction, grade } = options || {};

	let query = client
		.from('signals')
		.select('*')
		.order('created_at', { ascending: false })
		.range(offset, offset + limit - 1);

	if (direction) {
		query = query.eq('direction', direction);
	}
	if (grade) {
		query = query.eq('grade', grade);
	}

	const { data, error } = await query;
	if (error) throw error;
	return data as Signal[];
}

/**
 * Subscribe to real-time signals
 */
export function subscribeToSignals(
	callback: (signal: Signal) => void,
	options?: {
		direction?: SignalDirection;
		grade?: SignalGrade;
	}
): RealtimeChannel {
	const client = getSupabaseClient();
	let channel = client.channel('signals-channel').on(
		'postgres_changes',
		{
			event: 'INSERT',
			schema: 'public',
			table: 'signals'
		},
		(payload) => {
			const signal = payload.new as Signal;
			// Apply filters if specified
			if (options?.direction && signal.direction !== options.direction) return;
			if (options?.grade && signal.grade !== options.grade) return;
			callback(signal);
		}
	);

	channel.subscribe();
	return channel;
}

/**
 * Subscribe to real-time donations
 */
export function subscribeToDonations(callback: (donation: Donation) => void): RealtimeChannel {
	const client = getSupabaseClient();
	const channel = client.channel('donations-channel').on(
		'postgres_changes',
		{
			event: 'INSERT',
			schema: 'public',
			table: 'donations'
		},
		(payload) => {
			callback(payload.new as Donation);
		}
	);

	channel.subscribe();
	return channel;
}

// =============================================================================
// SPONSORS API
// =============================================================================

/**
 * Get active sponsors
 */
export async function getSponsors(activeOnly: boolean = true) {
	const client = getSupabaseClient();

	let query = client.from('sponsors').select('*').order('display_order', { ascending: true });

	if (activeOnly) {
		query = query.eq('active', true);
	}

	const { data, error } = await query;
	if (error) throw error;
	return data as Sponsor[];
}

// =============================================================================
// SONG REQUESTS API
// =============================================================================

/**
 * Get song requests
 */
export async function getSongRequests(options?: { limit?: number; status?: string }) {
	const client = getSupabaseClient();
	const { limit = 50, status } = options || {};

	let query = client
		.from('song_requests')
		.select('*')
		.order('created_at', { ascending: false })
		.limit(limit);

	if (status) {
		query = query.eq('status', status);
	}

	const { data, error } = await query;
	if (error) throw error;
	return data as SongRequest[];
}

/**
 * Submit a song request
 */
export async function submitSongRequest(request: {
	requester_name: string;
	song_title: string;
	artist: string;
}) {
	const client = getSupabaseClient();

	const { data, error } = await client
		.from('song_requests')
		.insert({
			requester_name: request.requester_name,
			song_title: request.song_title,
			artist: request.artist,
			status: 'pending'
		})
		.select()
		.single();

	if (error) throw error;
	return data as SongRequest;
}

// =============================================================================
// CLEANUP
// =============================================================================

/**
 * Unsubscribe from a channel
 */
export async function unsubscribe(channel: RealtimeChannel) {
	await channel.unsubscribe();
}

/**
 * Unsubscribe from all channels
 */
export async function unsubscribeAll() {
	const client = getSupabaseClient();
	await client.removeAllChannels();
}

// Export client for advanced usage
export { supabase };
export default {
	initSupabase,
	getSupabaseClient,
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
	unsubscribe,
	unsubscribeAll
};
