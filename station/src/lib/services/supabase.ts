import { createClient } from '@supabase/supabase-js';
import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY } from '$env/static/public';

if (!PUBLIC_SUPABASE_URL || !PUBLIC_SUPABASE_ANON_KEY) {
	throw new Error('Missing Supabase environment variables');
}

export const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY);

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
