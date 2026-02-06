/**
 * BaconAlgo 2040 - Auth Middleware
 * Protection des routes et gestion de session
 */

import { redirect } from '@sveltejs/kit';
import type { Handle } from '@sveltejs/kit';
import { getSupabaseClient } from '$lib/supabase/client';

/**
 * Hook de gestion de l'authentification SvelteKit
 */
export const handle: Handle = async ({ event, resolve }) => {
	const supabase = getSupabaseClient();
	
	// Vérifier la session
	const { data: { session } } = await supabase.auth.getSession();
	
	// Stocker la session dans les locals pour accès dans les routes
	event.locals.session = session;
	event.locals.user = session?.user ?? null;
	
	return resolve(event);
};

/**
 * Routes protégées - nécessitent authentification
 */
const PROTECTED_ROUTES = [
	'/dashboard',
	'/portfolio',
	'/settings',
	'/admin'
];

/**
 * Routes publiques - accessibles sans authentification
 */
const PUBLIC_ROUTES = [
	'/',
	'/login',
	'/register',
	'/pricing',
	'/forgot-password'
];

/**
 * Vérifier si une route nécessite l'authentification
 */
export function isProtectedRoute(pathname: string): boolean {
	return PROTECTED_ROUTES.some(route => pathname.startsWith(route));
}

/**
 * Vérifier si une route est publique
 */
export function isPublicRoute(pathname: string): boolean {
	return PUBLIC_ROUTES.some(route => pathname === route || pathname.startsWith(route));
}

/**
 * Guard pour les routes protégées (à utiliser dans +page.ts)
 */
export async function requireAuth() {
	const supabase = getSupabaseClient();
	const { data: { session } } = await supabase.auth.getSession();
	
	if (!session) {
		throw redirect(303, '/login');
	}
	
	return { session };
}

/**
 * Guard pour les routes admin (nécessite tier INSTITUTIONAL)
 */
export async function requireAdmin() {
	const supabase = getSupabaseClient();
	const { data: { session } } = await supabase.auth.getSession();
	
	if (!session) {
		throw redirect(303, '/login');
	}
	
	const { data: { user } } = await supabase.auth.getUser();
	const tier = user?.user_metadata?.subscription_tier;
	
	if (tier !== 'INSTITUTIONAL') {
		throw redirect(303, '/dashboard');
	}
	
	return { session };
}

/**
 * Guard pour les routes Station (nécessite tier STATION ou supérieur)
 */
export async function requireStation() {
	const supabase = getSupabaseClient();
	const { data: { session } } = await supabase.auth.getSession();
	
	if (!session) {
		throw redirect(303, '/login');
	}
	
	const { data: { user } } = await supabase.auth.getUser();
	const tier = user?.user_metadata?.subscription_tier || 'FREE';
	
	const tierLevels: Record<string, number> = {
		FREE: 0,
		STATION: 1,
		PRO: 2,
		INSTITUTIONAL: 3
	};
	
	if (tierLevels[tier] < tierLevels['STATION']) {
		throw redirect(303, '/pricing');
	}
	
	return { session };
}

/**
 * Rediriger les utilisateurs connectés loin des pages publiques
 */
export async function redirectIfAuthenticated() {
	const supabase = getSupabaseClient();
	const { data: { session } } = await supabase.auth.getSession();
	
	if (session) {
		throw redirect(303, '/dashboard');
	}
}
