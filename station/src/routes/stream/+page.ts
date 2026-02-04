import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
	const widget = url.searchParams.get('widget') || 'dashboard';
	
	return {
		widget,
		meta: {
			title: 'BaconAlgo Stream Overlay',
			description: 'OBS-ready stream overlay for BaconAlgo trading signals'
		}
	};
};
