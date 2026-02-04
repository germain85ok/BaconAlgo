/**
 * Centralized Environment Configuration
 * Loads and validates all environment variables
 */

// Social Links
export const YOUTUBE_URL = import.meta.env.PUBLIC_YOUTUBE_URL || 'https://youtube.com/@baconalgo';
export const DISCORD_URL = import.meta.env.PUBLIC_DISCORD_URL || 'https://discord.gg/baconalgo';

// Donation Links
export const PAYPAL_DONATE_URL = import.meta.env.PUBLIC_PAYPAL_DONATE_URL || '';
export const STRIPE_DONATE_URL = import.meta.env.PUBLIC_STRIPE_DONATE_URL || '';

// Crypto Addresses
export const BITCOIN_ADDRESS = import.meta.env.PUBLIC_BITCOIN_ADDRESS || '';
export const ETHEREUM_ADDRESS = import.meta.env.PUBLIC_ETHEREUM_ADDRESS || '';
export const SOLANA_ADDRESS = import.meta.env.PUBLIC_SOLANA_ADDRESS || '';
export const USDT_ADDRESS = import.meta.env.PUBLIC_USDT_ADDRESS || '';
export const USDC_ADDRESS = import.meta.env.PUBLIC_USDC_ADDRESS || '';

// Supabase
export const SUPABASE_URL = import.meta.env.PUBLIC_SUPABASE_URL || '';
export const SUPABASE_ANON_KEY = import.meta.env.PUBLIC_SUPABASE_ANON_KEY || '';

// Backend API
export const BACKEND_URL = import.meta.env.PUBLIC_BACKEND_URL || 'http://localhost:8080';

// Branding
export const BRAND_NAME = import.meta.env.PUBLIC_BRAND_NAME || 'BaconAlgo';
export const BRAND_TAGLINE = import.meta.env.PUBLIC_BRAND_TAGLINE || 'Your Trading Scanner & Stream Platform';
export const BRAND_COLOR = import.meta.env.PUBLIC_BRAND_COLOR || '#FF6B35';
export const ACCENT_COLOR = import.meta.env.PUBLIC_ACCENT_COLOR || '#F7931E';

// Donation Tiers (in USD)
export const DONATION_TIERS = {
	BACON_BIT: parseFloat(import.meta.env.TIER_BACON_BIT || '2'),
	CRISPY: parseFloat(import.meta.env.TIER_CRISPY || '5'),
	SIZZLING: parseFloat(import.meta.env.TIER_SIZZLING || '10'),
	LEGENDARY: parseFloat(import.meta.env.TIER_LEGENDARY || '20')
};

// Donation Settings
export const DAILY_DONATION_GOAL = parseFloat(import.meta.env.DAILY_DONATION_GOAL || '100');
export const MIN_DONATION_ALERT = parseFloat(import.meta.env.MIN_DONATION_ALERT || '2');
export const ALERT_DURATION = parseInt(import.meta.env.ALERT_DURATION || '10', 10);

// Feature Flags
export const ENABLE_DONATIONS = import.meta.env.ENABLE_DONATIONS !== 'false';
export const ENABLE_SPONSORS = import.meta.env.ENABLE_SPONSORS !== 'false';
export const ENABLE_SONG_REQUESTS = import.meta.env.ENABLE_SONG_REQUESTS !== 'false';
export const ENABLE_CHAT_OVERLAY = import.meta.env.ENABLE_CHAT_OVERLAY !== 'false';

// Contact
export const CONTACT_EMAIL = import.meta.env.CONTACT_EMAIL || 'sponsors@baconalgo.com';
export const CONTACT_DISCORD = import.meta.env.CONTACT_DISCORD || 'BaconAlgo#0000';

/**
 * Validate required environment variables
 */
export function validateEnv(): { valid: boolean; missing: string[] } {
	const required = [
		{ key: 'PUBLIC_SUPABASE_URL', value: SUPABASE_URL },
		{ key: 'PUBLIC_SUPABASE_ANON_KEY', value: SUPABASE_ANON_KEY }
	];

	const missing = required.filter((item) => !item.value).map((item) => item.key);

	return {
		valid: missing.length === 0,
		missing
	};
}

/**
 * Get donation tier info by amount
 */
export function getDonationTier(amount: number): {
	name: string;
	color: string;
	emoji: string;
} {
	if (amount >= DONATION_TIERS.LEGENDARY) {
		return { name: 'Legendary', color: '#FFD700', emoji: '🏆' };
	} else if (amount >= DONATION_TIERS.SIZZLING) {
		return { name: 'Sizzling', color: '#FF6B35', emoji: '🔥' };
	} else if (amount >= DONATION_TIERS.CRISPY) {
		return { name: 'Crispy', color: '#F7931E', emoji: '✨' };
	} else {
		return { name: 'Bacon Bit', color: '#FFA500', emoji: '🥓' };
	}
}

export default {
	YOUTUBE_URL,
	DISCORD_URL,
	PAYPAL_DONATE_URL,
	STRIPE_DONATE_URL,
	BITCOIN_ADDRESS,
	ETHEREUM_ADDRESS,
	SOLANA_ADDRESS,
	USDT_ADDRESS,
	USDC_ADDRESS,
	SUPABASE_URL,
	SUPABASE_ANON_KEY,
	BACKEND_URL,
	BRAND_NAME,
	BRAND_TAGLINE,
	BRAND_COLOR,
	ACCENT_COLOR,
	DONATION_TIERS,
	DAILY_DONATION_GOAL,
	MIN_DONATION_ALERT,
	ALERT_DURATION,
	ENABLE_DONATIONS,
	ENABLE_SPONSORS,
	ENABLE_SONG_REQUESTS,
	ENABLE_CHAT_OVERLAY,
	CONTACT_EMAIL,
	CONTACT_DISCORD,
	validateEnv,
	getDonationTier
};
