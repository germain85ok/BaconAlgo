// Coupon and subscription service
import { supabase } from './supabase';

export interface Coupon {
	id: string;
	code: string;
	discount_percent: number | null;
	discount_amount: number | null;
	valid_from: string;
	valid_until: string | null;
	max_uses: number | null;
	current_uses: number;
	applicable_plans: string[];
	is_active: boolean;
}

export interface CouponValidationResult {
	valid: boolean;
	discount_percent: number | null;
	discount_amount: number | null;
	message: string;
}

export interface Subscription {
	id: string;
	user_id: string;
	plan: 'FREE' | 'INDICATEUR' | 'SCANNER' | 'STATION';
	status: 'active' | 'cancelled' | 'expired' | 'past_due';
	billing_cycle: 'monthly' | 'yearly';
	amount: number;
	coupon_used: string | null;
	stripe_subscription_id: string | null;
	stripe_customer_id: string | null;
	current_period_start: string;
	current_period_end: string;
	cancelled_at: string | null;
	created_at: string;
	updated_at: string;
}

// Pricing structure
export const PRICING = {
	FREE: { monthly: 0, yearly: 0 },
	INDICATEUR: { monthly: 20, yearly: 192 }, // 20% discount
	SCANNER: { monthly: 30, yearly: 288 }, // 20% discount
	STATION: { monthly: 50, yearly: 480 } // 20% discount
} as const;

/**
 * Calculate price with coupon discount
 */
export function calculatePriceWithDiscount(
	plan: keyof typeof PRICING,
	billingCycle: 'monthly' | 'yearly',
	coupon?: CouponValidationResult
): number {
	const basePrice = PRICING[plan][billingCycle];

	if (!coupon || !coupon.valid) {
		return basePrice;
	}

	if (coupon.discount_percent !== null) {
		return basePrice * (1 - coupon.discount_percent / 100);
	}

	if (coupon.discount_amount !== null) {
		return Math.max(0, basePrice - coupon.discount_amount);
	}

	return basePrice;
}

/**
 * Validate a coupon code for a specific plan
 */
export async function validateCoupon(
	code: string,
	plan: string
): Promise<CouponValidationResult> {
	try {
		const { data, error } = await supabase.rpc('validate_coupon', {
			coupon_code: code,
			plan_name: plan
		});

		if (error) {
			console.error('Error validating coupon:', error);
			return {
				valid: false,
				discount_percent: null,
				discount_amount: null,
				message: 'Error validating coupon'
			};
		}

		if (data && data.length > 0) {
			return data[0];
		}

		return {
			valid: false,
			discount_percent: null,
			discount_amount: null,
			message: 'Invalid coupon'
		};
	} catch (err) {
		console.error('Exception validating coupon:', err);
		return {
			valid: false,
			discount_percent: null,
			discount_amount: null,
			message: 'Error validating coupon'
		};
	}
}

/**
 * Create a new subscription
 */
export async function createSubscription(subscription: {
	user_id: string;
	plan: string;
	billing_cycle: 'monthly' | 'yearly';
	amount: number;
	coupon_used?: string;
	stripe_subscription_id?: string;
	stripe_customer_id?: string;
}): Promise<{ data: Subscription | null; error: any }> {
	const period_start = new Date();
	const period_end = new Date();

	if (subscription.billing_cycle === 'monthly') {
		period_end.setMonth(period_end.getMonth() + 1);
	} else {
		period_end.setFullYear(period_end.getFullYear() + 1);
	}

	const { data, error } = await supabase
		.from('subscriptions')
		.insert({
			...subscription,
			status: 'active',
			current_period_start: period_start.toISOString(),
			current_period_end: period_end.toISOString()
		})
		.select()
		.single();

	// Increment coupon usage if a coupon was used
	if (!error && subscription.coupon_used) {
		await supabase.rpc('increment_coupon_usage', {
			coupon_code: subscription.coupon_used
		});
	}

	return { data, error };
}

/**
 * Get user's active subscription
 */
export async function getUserSubscription(
	userId: string
): Promise<{ data: Subscription | null; error: any }> {
	const { data, error } = await supabase
		.from('subscriptions')
		.select('*')
		.eq('user_id', userId)
		.eq('status', 'active')
		.order('created_at', { ascending: false })
		.limit(1)
		.single();

	return { data, error };
}

/**
 * Cancel a subscription
 */
export async function cancelSubscription(
	subscriptionId: string
): Promise<{ data: any; error: any }> {
	const { data, error } = await supabase
		.from('subscriptions')
		.update({
			status: 'cancelled',
			cancelled_at: new Date().toISOString()
		})
		.eq('id', subscriptionId)
		.select()
		.single();

	return { data, error };
}

/**
 * Get all subscriptions (admin only)
 */
export async function getAllSubscriptions(): Promise<{
	data: Subscription[] | null;
	error: any;
}> {
	const { data, error } = await supabase
		.from('subscriptions')
		.select('*, profiles(email, username)')
		.order('created_at', { ascending: false });

	return { data, error };
}

/**
 * Get all coupons (admin only)
 */
export async function getAllCoupons(): Promise<{ data: Coupon[] | null; error: any }> {
	const { data, error } = await supabase
		.from('coupons')
		.select('*')
		.order('created_at', { ascending: false });

	return { data, error };
}

/**
 * Create a new coupon (admin only)
 */
export async function createCoupon(coupon: {
	code: string;
	discount_percent?: number;
	discount_amount?: number;
	valid_until?: string;
	max_uses?: number;
	applicable_plans?: string[];
}): Promise<{ data: Coupon | null; error: any }> {
	const { data, error } = await supabase.from('coupons').insert(coupon).select().single();

	return { data, error };
}

/**
 * Update a coupon (admin only)
 */
export async function updateCoupon(
	id: string,
	updates: Partial<Coupon>
): Promise<{ data: Coupon | null; error: any }> {
	const { data, error } = await supabase
		.from('coupons')
		.update(updates)
		.eq('id', id)
		.select()
		.single();

	return { data, error };
}

/**
 * Delete a coupon (admin only)
 */
export async function deleteCoupon(id: string): Promise<{ data: any; error: any }> {
	const { data, error } = await supabase.from('coupons').delete().eq('id', id);

	return { data, error };
}
