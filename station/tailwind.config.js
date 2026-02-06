/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'bacon-orange': '#ff6b35',
				'bacon-accent': '#ffb347',
				'bacon-glow': 'rgba(255, 107, 53, 0.3)',
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
			},
			boxShadow: {
				'glow-sm': '0 0 10px rgba(255, 107, 53, 0.3)',
				'glow-md': '0 0 20px rgba(255, 107, 53, 0.4)',
				'glow-lg': '0 0 30px rgba(255, 107, 53, 0.5)',
				'glow-xl': '0 0 40px rgba(255, 107, 53, 0.6)'
			},
			backgroundImage: {
				'gradient-bacon': 'linear-gradient(to right, #ff6b35, #f97316)',
				'gradient-dark': 'linear-gradient(135deg, #0a0a0f 0%, #1a1f3a 100%)',
				'gradient-radial': 'radial-gradient(circle, var(--tw-gradient-stops))'
			},
			animation: {
				'pulse-glow': 'pulse-glow 2s ease-in-out infinite',
				'shimmer': 'shimmer 2s infinite',
				'neon-border': 'neon-border 2s ease-in-out infinite',
				'float': 'float 3s ease-in-out infinite',
				'slide-in-left': 'slide-in-left 0.5s ease-out',
				'slide-in-right': 'slide-in-right 0.5s ease-out',
				'fade-in': 'fade-in 0.5s ease-out',
				'scale-in': 'scale-in 0.5s ease-out'
			}
		}
	},
	plugins: [require('@tailwindcss/forms')]
};
