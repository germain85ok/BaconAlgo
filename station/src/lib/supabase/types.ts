// Additional SAAS Platform Types

export type UserPlan = 'free' | 'indicator' | 'scanner' | 'station';

export type PromoCodeType = 'full_access' | 'trial' | 'discount';

export type ReferralStatus = 'pending' | 'converted' | 'paid';

export type PositionSide = 'long' | 'short';

export type PositionStatus = 'open' | 'closed';

export interface Profile {
	id: string;
	email: string | null;
	username: string | null;
	avatar_url: string | null;
	plan: UserPlan;
	plan_expires_at: string | null;
	trial_ends_at: string | null;
	promo_code_used: string | null;
	referral_code: string;
	referred_by: string | null;
	bacon_points: number;
	badges: string[];
	streak_days: number;
	last_active_at: string | null;
	is_admin: boolean;
	is_banned: boolean;
	two_factor_enabled: boolean;
	created_at: string;
	updated_at: string;
}

export interface PromoCode {
	id: string;
	code: string;
	type: PromoCodeType;
	discount_percent: number | null;
	trial_days: number | null;
	plan_grant: UserPlan | null;
	max_uses: number | null;
	current_uses: number;
	expires_at: string | null;
	is_active: boolean;
	created_at: string;
}

export interface Referral {
	id: string;
	referrer_id: string;
	referred_id: string;
	status: ReferralStatus;
	reward_given: boolean;
	created_at: string;
}

export interface Achievement {
	id: string;
	name: string;
	description: string | null;
	icon: string | null;
	points: number;
	criteria: Record<string, any>;
}

export interface WatchlistItem {
	id: string;
	user_id: string;
	symbol: string;
	notes: string | null;
	added_at: string;
}

export interface PaperPosition {
	id: string;
	user_id: string;
	symbol: string;
	side: PositionSide;
	quantity: number;
	entry_price: number;
	current_price: number | null;
	stop_loss: number | null;
	take_profit: number | null;
	pnl: number | null;
	pnl_percent: number | null;
	status: PositionStatus;
	opened_at: string;
	closed_at: string | null;
}

export interface AcademyProgress {
	id: string;
	user_id: string;
	course_id: string;
	lesson_id: string;
	completed: boolean;
	quiz_score: number | null;
	completed_at: string | null;
}

export interface UserAchievement {
	id: string;
	user_id: string;
	achievement_id: string;
	earned_at: string;
}

// Extended Signal type with more details
export interface ExtendedSignal {
	id: string;
	ticker: string;
	direction: 'BUY' | 'SELL';
	grade: 'S' | 'A' | 'B' | 'C';
	score: number;
	entry_price?: number;
	stop_loss?: number;
	take_profit_1?: number;
	take_profit_2?: number;
	take_profit_3?: number;
	risk_reward_ratio?: number;
	smc_tags?: string[]; // FVG, OB, BOS, CHoCH, etc.
	horizon?: 'scalp' | 'day' | 'swing' | 'long';
	indicators: Record<string, any>;
	created_at: string;
}
