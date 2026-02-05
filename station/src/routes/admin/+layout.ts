import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { getSupabaseClient } from '$lib/supabase/client';

export const load: LayoutLoad = async ({ url }) => {
	const supabase = getSupabaseClient();

	const {
		data: { session }
	} = await supabase.auth.getSession();

	if (!session) {
		throw redirect(303, `/login?redirectTo=${encodeURIComponent(url.pathname)}`);
	}

	const { data: profile, error } = await supabase
		.from('profiles')
		.select('*')
		.eq('id', session.user.id)
		.single();

	if (error || !profile) {
		throw redirect(303, '/login');
	}

	// Admin guard - only admins can access
	if (!profile.is_admin) {
		throw redirect(303, '/dashboard');
	}

	return {
		session,
		profile
	};
};
