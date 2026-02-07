import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		// Use adapter-node for production SSR deployment
		adapter: adapter({
			out: 'build',
			precompress: true,
			envPrefix: 'PUBLIC_'
		})
	}
};

export default config;
