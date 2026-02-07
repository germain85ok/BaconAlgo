import { createClient } from '@supabase/supabase-js';
import { SUPABASE_URL, SUPABASE_ANON_KEY } from '../config/env';

if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
	throw new Error('Missing Supabase environment variables');
}

export const supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY);

// Helper to get the current user
export async function getCurrentUser() {
	const {
		data: { user },
		error
	} = await supabase.auth.getUser();
	return { user, error };
}

// Helper to check if user is admin
export async function isUserAdmin() {
	const {
		data: { user }
	} = await supabase.auth.getUser();

	if (!user) return false;

	const { data: profile } = await supabase
		.from('profiles')
		.select('is_admin')
		.eq('id', user.id)
		.single();

	return profile?.is_admin === true;
}

// Helper to get user profile
export async function getUserProfile(userId: string) {
	const { data, error } = await supabase
		.from('profiles')
		.select('*')
		.eq('id', userId)
		.single();

	return { data, error };
}
