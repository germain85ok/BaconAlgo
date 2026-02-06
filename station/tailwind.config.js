/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				// Primary colors
				'bacon-orange': '#ff6b35',
				'bacon-red': '#ff4d6a',
				'bacon-light': '#ffb347',
				'bacon-dark': '#f7931e',

				// Accent colors
				'success-cyan': '#00d9ff',
				'accent-purple': '#a855f7',
				'accent-pink': '#ec4899',
				'warning-yellow': '#ffd93d',

				// Status colors
				'success': '#22c55e',
				'warning': '#eab308',
				'error': '#ef4444',
				'info': '#3b82f6',

				// Background colors
				'bg-dark': '#0a0a0f',
				'bg-darker': '#050508',
				'bg-card': 'rgba(255, 255, 255, 0.05)',

				// Text colors
				'text-primary': '#ffffff',
				'text-secondary': '#a0a0a0',
				'text-tertiary': '#6b6b6b',

				// Trading colors
				'bullish': '#22c55e',
				'bearish': '#ef4444',
				'neutral': '#a0a0a0'
			},
			fontFamily: {
				display: ['Orbitron', 'sans-serif'],
				body: ['Inter', 'sans-serif'],
				mono: ['JetBrains Mono', 'monospace']
			},
			backdropBlur: {
				xs: '2px',
				'2xl': '40px',
				'3xl': '64px'
			},
			backgroundImage: {
				'gradient-primary': 'linear-gradient(135deg, #ff6b35 0%, #f7931e 100%)',
				'gradient-primary-vertical': 'linear-gradient(180deg, #ff6b35 0%, #f7931e 100%)',
				'gradient-holographic':
					'linear-gradient(135deg, #ff6b35 0%, #00d9ff 25%, #a855f7 50%, #ff6b35 75%, #00d9ff 100%)',
				'gradient-animated':
					'linear-gradient(135deg, #0a0a0f 0%, #1a0f1a 50%, #0a0a0f 100%)',
				'gradient-text': 'linear-gradient(135deg, #ff6b35 0%, #ffb347 50%, #f7931e 100%)'
			},
			boxShadow: {
				'glow-sm': '0 0 10px rgba(255, 107, 53, 0.5)',
				'glow-md': '0 0 20px rgba(255, 107, 53, 0.6), 0 0 40px rgba(255, 107, 53, 0.4)',
				'glow-lg':
					'0 0 30px rgba(255, 107, 53, 0.7), 0 0 60px rgba(255, 107, 53, 0.5), 0 0 90px rgba(255, 107, 53, 0.3)',
				'glow-xl':
					'0 0 40px rgba(255, 107, 53, 0.8), 0 0 80px rgba(255, 107, 53, 0.6), 0 0 120px rgba(255, 107, 53, 0.4)',
				'glow-cyan-sm': '0 0 10px rgba(0, 217, 255, 0.5)',
				'glow-cyan-md': '0 0 20px rgba(0, 217, 255, 0.6), 0 0 40px rgba(0, 217, 255, 0.4)',
				'glow-green-sm': '0 0 10px rgba(34, 197, 94, 0.5)',
				'glow-red-sm': '0 0 10px rgba(239, 68, 68, 0.5)',
				glass: '0 8px 32px rgba(0, 0, 0, 0.3)',
				'glass-lg': '0 12px 48px rgba(0, 0, 0, 0.4)'
			},
			animation: {
				'pulse-glow': 'pulseGlow 2s ease-in-out infinite',
				shimmer: 'shimmer 3s linear infinite',
				'neon-border': 'neonBorder 3s linear infinite',
				float: 'float 3s ease-in-out infinite',
				'fade-in': 'fadeIn 0.5s ease-out',
				'slide-in-right': 'slideInRight 0.5s ease-out',
				'slide-in-left': 'slideInLeft 0.5s ease-out',
				'slide-in-up': 'slideInUp 0.5s ease-out',
				'slide-in-down': 'slideInDown 0.5s ease-out'
			},
			keyframes: {
				pulseGlow: {
					'0%, 100%': { boxShadow: '0 0 20px rgba(255, 107, 53, 0.5)' },
					'50%': { boxShadow: '0 0 40px rgba(255, 107, 53, 0.8)' }
				},
				shimmer: {
					'0%': { backgroundPosition: '-200% center' },
					'100%': { backgroundPosition: '200% center' }
				},
				neonBorder: {
					'0%': {
						borderColor: '#ff6b35',
						boxShadow: '0 0 10px rgba(255, 107, 53, 0.5)'
					},
					'33%': {
						borderColor: '#00d9ff',
						boxShadow: '0 0 10px rgba(0, 217, 255, 0.5)'
					},
					'66%': {
						borderColor: '#a855f7',
						boxShadow: '0 0 10px rgba(168, 85, 247, 0.5)'
					},
					'100%': {
						borderColor: '#ff6b35',
						boxShadow: '0 0 10px rgba(255, 107, 53, 0.5)'
					}
				},
				float: {
					'0%, 100%': { transform: 'translateY(0px)' },
					'50%': { transform: 'translateY(-10px)' }
				},
				fadeIn: {
					from: { opacity: '0', transform: 'translateY(20px)' },
					to: { opacity: '1', transform: 'translateY(0)' }
				},
				slideInRight: {
					from: { opacity: '0', transform: 'translateX(50px)' },
					to: { opacity: '1', transform: 'translateX(0)' }
				},
				slideInLeft: {
					from: { opacity: '0', transform: 'translateX(-50px)' },
					to: { opacity: '1', transform: 'translateX(0)' }
				},
				slideInUp: {
					from: { opacity: '0', transform: 'translateY(50px)' },
					to: { opacity: '1', transform: 'translateY(0)' }
				},
				slideInDown: {
					from: { opacity: '0', transform: 'translateY(-50px)' },
					to: { opacity: '1', transform: 'translateY(0)' }
				}
			},
			borderRadius: {
				'4xl': '2rem'
			}
		}
	},
	plugins: [require('@tailwindcss/forms')]
};
