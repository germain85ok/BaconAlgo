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

	// Admin guard - only admins can access (is_admin flag OR specific email)
	const isAdmin = profile.is_admin || profile.email === 'germain85@hotmail.com';
	
	if (!isAdmin) {
		throw redirect(303, '/dashboard');
	}

	return {
		session,
		profile
	};
};
