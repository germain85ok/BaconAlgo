import { writable, derived } from 'svelte/store';
import type { User, SubscriptionTier } from '$lib/types';

export type TierLevel = 'FREE' | 'STATION' | 'PRO' | 'INSTITUTIONAL';

const PROMO_CODES = {
	'ILOVEBACON-AND-TEA': 'INSTITUTIONAL',
	'BACONALGO2040': 'PRO',
	'PRO2040': 'PRO'
} as const;

const TIER_FEATURES: Record<TierLevel, SubscriptionTier> = {
	FREE: {
		tier: 'FREE',
		features: [
			'1 paire de trading',
			'Signaux limités (5/jour)',
			'1 timeframe',
			'Support communautaire'
		],
		max_signals: 5,
		max_timeframes: 1,
		has_api_access: false,
		has_webhooks: false
	},
	STATION: {
		tier: 'STATION',
		features: [
			'5 paires de trading',
			'Signaux illimités',
			'3 timeframes',
			'Indicateurs de base',
			'Support par email',
			'Overlay de stream'
		],
		max_signals: -1,
		max_timeframes: 3,
		has_api_access: false,
		has_webhooks: false
	},
	PRO: {
		tier: 'PRO',
		features: [
			'Paires illimitées',
			'Tous les timeframes',
			'Indicateurs avancés',
			'Neural AI Score',
			'API REST',
			'Webhooks',
			'Support prioritaire',
			'Backtesting',
			'Auto-trading'
		],
		max_signals: -1,
		max_timeframes: 7,
		has_api_access: true,
		has_webhooks: true
	},
	INSTITUTIONAL: {
		tier: 'INSTITUTIONAL',
		features: [
			'Accès complet',
			'Multi-comptes',
			'API dédiée',
			'Webhooks illimités',
			'Support 24/7',
			'Formation personnalisée',
			'Intégration custom',
			'Audit de stratégie',
			'Exécution haute fréquence'
		],
		max_signals: -1,
		max_timeframes: 7,
		has_api_access: true,
		has_webhooks: true
	}
};

interface AuthState {
	user: User | null;
	isAuthenticated: boolean;
	isLoading: boolean;
	error: string | null;
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>({
		user: null,
		isAuthenticated: false,
		isLoading: true,
		error: null
	});

	if (typeof window !== 'undefined') {
		const savedUser = localStorage.getItem('baconalgo_user');
		if (savedUser) {
			try {
				const user = JSON.parse(savedUser);
				set({
					user,
					isAuthenticated: true,
					isLoading: false,
					error: null
				});
			} catch (error) {
				set({
					user: null,
					isAuthenticated: false,
					isLoading: false,
					error: null
				});
			}
		} else {
			update((state) => ({ ...state, isLoading: false }));
		}
	}

	return {
		subscribe,
		login: async (email: string, password: string) => {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				const response = await fetch('/api/auth/login', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ email, password })
				});

				if (!response.ok) {
					const error = await response.json();
					throw new Error(error.message || 'Erreur de connexion');
				}

				const data = await response.json();
				const user: User = data.user;

				localStorage.setItem('baconalgo_token', data.token);
				localStorage.setItem('baconalgo_user', JSON.stringify(user));

				set({
					user,
					isAuthenticated: true,
					isLoading: false,
					error: null
				});

				return { success: true };
			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Erreur de connexion';
				update((state) => ({
					...state,
					isLoading: false,
					error: errorMessage
				}));
				return { success: false, error: errorMessage };
			}
		},

		register: async (email: string, username: string, password: string, promoCode?: string) => {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				let tier: TierLevel = 'FREE';

				if (promoCode && promoCode in PROMO_CODES) {
					tier = PROMO_CODES[promoCode as keyof typeof PROMO_CODES];
				}

				const response = await fetch('/api/auth/register', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ email, username, password, tier })
				});

				if (!response.ok) {
					const error = await response.json();
					throw new Error(error.message || "Erreur d'inscription");
				}

				const data = await response.json();
				const user: User = data.user;

				localStorage.setItem('baconalgo_token', data.token);
				localStorage.setItem('baconalgo_user', JSON.stringify(user));

				set({
					user,
					isAuthenticated: true,
					isLoading: false,
					error: null
				});

				return { success: true, tier };
			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : "Erreur d'inscription";
				update((state) => ({
					...state,
					isLoading: false,
					error: errorMessage
				}));
				return { success: false, error: errorMessage };
			}
		},

		logout: () => {
			localStorage.removeItem('baconalgo_token');
			localStorage.removeItem('baconalgo_user');

			set({
				user: null,
				isAuthenticated: false,
				isLoading: false,
				error: null
			});
		},

		oauthLogin: async (provider: 'google' | 'discord' | 'github') => {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				window.location.href = `/api/auth/oauth/${provider}`;
			} catch (error) {
				const errorMessage =
					error instanceof Error ? error.message : 'Erreur de connexion OAuth';
				update((state) => ({
					...state,
					isLoading: false,
					error: errorMessage
				}));
			}
		},

		clearError: () => {
			update((state) => ({ ...state, error: null }));
		}
	};
}

export const auth = createAuthStore();

export const currentTier = derived(auth, ($auth) => {
	if (!$auth.user) return TIER_FEATURES.FREE;
	return $auth.user.subscription;
});

export const hasFeature = derived(currentTier, ($tier) => {
	return {
		apiAccess: $tier.has_api_access,
		webhooks: $tier.has_webhooks,
		unlimitedSignals: $tier.max_signals === -1,
		allTimeframes: $tier.max_timeframes === 7
	};
});

export { TIER_FEATURES, PROMO_CODES };
