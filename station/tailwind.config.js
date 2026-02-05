/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'bacon-orange': '#ff6b35',
				'bacon-red': '#ff4d6a',
				'success-cyan': '#00d9ff',
				'warning-yellow': '#ffd93d',
				'bg-dark': '#0a0a0f',
				'bg-card': 'rgba(255, 255, 255, 0.05)',
				'text-primary': '#ffffff',
				'text-secondary': '#a0a0a0'
			},
			fontFamily: {
				display: ['Orbitron', 'sans-serif'],
				body: ['Inter', 'sans-serif'],
				mono: ['JetBrains Mono', 'monospace']
			},
			backdropBlur: {
				xs: '2px'
			}
		}
	},
	plugins: [require('@tailwindcss/forms')]
};
