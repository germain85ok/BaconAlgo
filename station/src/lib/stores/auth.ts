/**
 * BaconAlgo 2040 - Auth Store
 * Gestion de l'authentification avec Supabase et codes promo
 */

import { writable, derived } from 'svelte/store';
import { getSupabaseClient } from '$lib/supabase/client';
import type { User, Session } from '@supabase/supabase-js';

export interface SubscriptionTier {
	id: string;
	name: string;
	level: 'FREE' | 'STATION' | 'PRO' | 'INSTITUTIONAL';
	features: string[];
	price_monthly?: number;
	price_yearly?: number;
}

export interface UserProfile {
	id: string;
	email: string;
	subscription_tier: SubscriptionTier['level'];
	promo_code_used?: string;
	created_at: string;
	metadata?: Record<string, any>;
}

interface AuthState {
	user: User | null;
	session: Session | null;
	profile: UserProfile | null;
	loading: boolean;
	error: string | null;
}

const initialState: AuthState = {
	user: null,
	session: null,
	profile: null,
	loading: true,
	error: null
};

// Store principal
function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(initialState);

	const supabase = getSupabaseClient();

	// Codes promo et leurs tiers associés
	const PROMO_CODES: Record<string, SubscriptionTier['level']> = {
		'ILOVEBACON-AND-TEA': 'STATION',  // URL-safe version
		'BACONALGO2040': 'STATION',
		'PRO2040': 'PRO'
	};

	return {
		subscribe,

		/**
		 * Initialiser l'authentification
		 */
		async init() {
			update((state) => ({ ...state, loading: true }));

			try {
				// Vérifier la session existante
				const { data: { session }, error } = await supabase.auth.getSession();
				
				if (error) throw error;

				if (session?.user) {
					const profile = await loadUserProfile(session.user.id);
					update((state) => ({
						...state,
						user: session.user,
						session,
						profile,
						loading: false,
						error: null
					}));
				} else {
					update((state) => ({ ...state, loading: false }));
				}

				// Écouter les changements d'authentification
				supabase.auth.onAuthStateChange(async (event, session) => {
					if (event === 'SIGNED_IN' && session?.user) {
						const profile = await loadUserProfile(session.user.id);
						update((state) => ({
							...state,
							user: session.user,
							session,
							profile,
							error: null
						}));
					} else if (event === 'SIGNED_OUT') {
						update((state) => ({
							...state,
							user: null,
							session: null,
							profile: null
						}));
					}
				});
			} catch (error) {
				update((state) => ({
					...state,
					loading: false,
					error: error instanceof Error ? error.message : 'Erreur d\'initialisation'
				}));
			}
		},

		/**
		 * Connexion avec email/mot de passe
		 */
		async signIn(email: string, password: string) {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				const { data, error } = await supabase.auth.signInWithPassword({
					email,
					password
				});

				if (error) throw error;

				if (data.session?.user) {
					const profile = await loadUserProfile(data.session.user.id);
					update((state) => ({
						...state,
						user: data.session!.user,
						session: data.session,
						profile,
						loading: false,
						error: null
					}));
					return { success: true };
				}

				throw new Error('Aucune session créée');
			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Erreur de connexion';
				update((state) => ({ ...state, loading: false, error: errorMessage }));
				return { success: false, error: errorMessage };
			}
		},

		/**
		 * Inscription avec email/mot de passe et code promo optionnel
		 */
		async signUp(email: string, password: string, promoCode?: string) {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				// Vérifier le code promo
				let tier: SubscriptionTier['level'] = 'FREE';
				if (promoCode) {
					const upperCode = promoCode.toUpperCase().trim();
					if (PROMO_CODES[upperCode]) {
						tier = PROMO_CODES[upperCode];
					} else {
						throw new Error('Code promo invalide');
					}
				}

				const { data, error } = await supabase.auth.signUp({
					email,
					password,
					options: {
						data: {
							subscription_tier: tier,
							promo_code_used: promoCode?.toUpperCase().trim()
						}
					}
				});

				if (error) throw error;

				if (data.session?.user) {
					const profile = await loadUserProfile(data.session.user.id);
					update((state) => ({
						...state,
						user: data.session!.user,
						session: data.session,
						profile,
						loading: false,
						error: null
					}));
					return { success: true, requiresEmailVerification: !data.session };
				}

				// Si pas de session immédiate, vérification email requise
				update((state) => ({ ...state, loading: false }));
				return { success: true, requiresEmailVerification: true };
			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Erreur d\'inscription';
				update((state) => ({ ...state, loading: false, error: errorMessage }));
				return { success: false, error: errorMessage };
			}
		},

		/**
		 * Connexion avec OAuth (Google, Discord, etc.)
		 */
		async signInWithOAuth(provider: 'google' | 'discord' | 'github') {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				const { error } = await supabase.auth.signInWithOAuth({
					provider,
					options: {
						redirectTo: `${window.location.origin}/dashboard`
					}
				});

				if (error) throw error;

				return { success: true };
			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Erreur OAuth';
				update((state) => ({ ...state, loading: false, error: errorMessage }));
				return { success: false, error: errorMessage };
			}
		},

		/**
		 * Déconnexion
		 */
		async signOut() {
			try {
				await supabase.auth.signOut();
				set(initialState);
				return { success: true };
			} catch (error) {
				return {
					success: false,
					error: error instanceof Error ? error.message : 'Erreur de déconnexion'
				};
			}
		},

		/**
		 * Appliquer un code promo à un utilisateur existant
		 */
		async applyPromoCode(promoCode: string) {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				const upperCode = promoCode.toUpperCase().trim();
				if (!PROMO_CODES[upperCode]) {
					throw new Error('Code promo invalide');
				}

				const tier = PROMO_CODES[upperCode];

				// Mettre à jour le profil utilisateur dans Supabase
				const { data: { user } } = await supabase.auth.getUser();
				if (!user) throw new Error('Utilisateur non connecté');

				// Mettre à jour les métadonnées utilisateur
				const { error } = await supabase.auth.updateUser({
					data: {
						subscription_tier: tier,
						promo_code_used: upperCode
					}
				});

				if (error) throw error;

				// Recharger le profil
				const profile = await loadUserProfile(user.id);
				update((state) => ({
					...state,
					profile,
					loading: false,
					error: null
				}));

				return { success: true, tier };
			} catch (error) {
				const errorMessage =
					error instanceof Error ? error.message : 'Erreur d\'application du code promo';
				update((state) => ({ ...state, loading: false, error: errorMessage }));
				return { success: false, error: errorMessage };
			}
		},

		/**
		 * Réinitialiser l'erreur
		 */
		clearError() {
			update((state) => ({ ...state, error: null }));
		}
	};
}

/**
 * Charger le profil utilisateur depuis Supabase
 */
async function loadUserProfile(userId: string): Promise<UserProfile> {
	const supabase = getSupabaseClient();
	
	const { data: { user } } = await supabase.auth.getUser();
	
	if (!user) {
		throw new Error('Utilisateur non trouvé');
	}

	return {
		id: userId,
		email: user.email || '',
		subscription_tier: (user.user_metadata?.subscription_tier || 'FREE') as SubscriptionTier['level'],
		promo_code_used: user.user_metadata?.promo_code_used,
		created_at: user.created_at,
		metadata: user.user_metadata
	};
}

// Export du store
export const authStore = createAuthStore();

// Stores dérivés pour un accès facile
export const user = derived(authStore, ($auth) => $auth.user);
export const session = derived(authStore, ($auth) => $auth.session);
export const profile = derived(authStore, ($auth) => $auth.profile);
export const isAuthenticated = derived(authStore, ($auth) => !!$auth.user);
export const isLoading = derived(authStore, ($auth) => $auth.loading);

/**
 * Vérifier si l'utilisateur a accès à une fonctionnalité
 */
export const hasAccess = derived(authStore, ($auth) => {
	return (requiredTier: SubscriptionTier['level']) => {
		if (!$auth.profile) return false;

		const tierLevels: Record<SubscriptionTier['level'], number> = {
			FREE: 0,
			STATION: 1,
			PRO: 2,
			INSTITUTIONAL: 3
		};

		return tierLevels[$auth.profile.subscription_tier] >= tierLevels[requiredTier];
	};
});
