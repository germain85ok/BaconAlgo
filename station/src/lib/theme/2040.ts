/**
 * BaconAlgo 2040 - Design System
 * Système de design futuriste avec glassmorphism, néon orange et effets holographiques
 */

// ============================================================================
// PALETTE DE COULEURS 2040
// ============================================================================

export const colors = {
	// Couleurs primaires
	primary: {
		orange: '#ff6b35',
		orangeLight: '#ffb347',
		orangeDark: '#f7931e',
		glow: 'rgba(255, 107, 53, 0.3)'
	},

	// Couleurs d'accent
	accent: {
		cyan: '#00d9ff',
		purple: '#a855f7',
		pink: '#ec4899',
		yellow: '#ffd93d'
	},

	// Couleurs de statut
	status: {
		success: '#22c55e',
		warning: '#eab308',
		error: '#ef4444',
		info: '#3b82f6'
	},

	// Couleurs de fond
	background: {
		dark: '#0a0a0f',
		darker: '#050508',
		card: 'rgba(255, 255, 255, 0.05)',
		cardHover: 'rgba(255, 255, 255, 0.08)',
		overlay: 'rgba(0, 0, 0, 0.8)'
	},

	// Couleurs de texte
	text: {
		primary: '#ffffff',
		secondary: '#a0a0a0',
		tertiary: '#6b6b6b',
		accent: '#ff6b35'
	},

	// Trading colors
	trading: {
		bullish: '#22c55e',
		bearish: '#ef4444',
		neutral: '#a0a0a0'
	}
};

// ============================================================================
// GRADIENTS 2040
// ============================================================================

export const gradients = {
	// Gradient principal orange
	primary: 'linear-gradient(135deg, #ff6b35 0%, #f7931e 100%)',
	primaryVertical: 'linear-gradient(180deg, #ff6b35 0%, #f7931e 100%)',

	// Gradient de fond animé
	backgroundAnimated:
		'linear-gradient(135deg, #0a0a0f 0%, #1a0f1a 50%, #0a0a0f 100%)',

	// Gradients d'accent
	cyan: 'linear-gradient(135deg, #00d9ff 0%, #0099cc 100%)',
	purple: 'linear-gradient(135deg, #a855f7 0%, #7c3aed 100%)',
	gold: 'linear-gradient(135deg, #ffd700 0%, #ffb347 100%)',

	// Gradient holographique
	holographic:
		'linear-gradient(135deg, #ff6b35 0%, #00d9ff 25%, #a855f7 50%, #ff6b35 75%, #00d9ff 100%)',

	// Gradient pour le texte
	textGradient:
		'linear-gradient(135deg, #ff6b35 0%, #ffb347 50%, #f7931e 100%)'
};

// ============================================================================
// EFFETS DE GLOW (LUEUR NÉON)
// ============================================================================

export const glows = {
	// Glow orange (principal)
	orange: {
		small: '0 0 10px rgba(255, 107, 53, 0.5)',
		medium: '0 0 20px rgba(255, 107, 53, 0.6), 0 0 40px rgba(255, 107, 53, 0.4)',
		large: '0 0 30px rgba(255, 107, 53, 0.7), 0 0 60px rgba(255, 107, 53, 0.5), 0 0 90px rgba(255, 107, 53, 0.3)',
		intense:
			'0 0 40px rgba(255, 107, 53, 0.8), 0 0 80px rgba(255, 107, 53, 0.6), 0 0 120px rgba(255, 107, 53, 0.4)'
	},

	// Glow cyan
	cyan: {
		small: '0 0 10px rgba(0, 217, 255, 0.5)',
		medium: '0 0 20px rgba(0, 217, 255, 0.6), 0 0 40px rgba(0, 217, 255, 0.4)',
		large: '0 0 30px rgba(0, 217, 255, 0.7), 0 0 60px rgba(0, 217, 255, 0.5)'
	},

	// Glow vert (bullish)
	green: {
		small: '0 0 10px rgba(34, 197, 94, 0.5)',
		medium: '0 0 20px rgba(34, 197, 94, 0.6), 0 0 40px rgba(34, 197, 94, 0.4)'
	},

	// Glow rouge (bearish)
	red: {
		small: '0 0 10px rgba(239, 68, 68, 0.5)',
		medium: '0 0 20px rgba(239, 68, 68, 0.6), 0 0 40px rgba(239, 68, 68, 0.4)'
	}
};

// ============================================================================
// GLASSMORPHISM
// ============================================================================

export const glass = {
	// Base glassmorphism
	base: {
		background: 'rgba(255, 255, 255, 0.05)',
		backdropFilter: 'blur(20px)',
		border: '1px solid rgba(255, 255, 255, 0.1)',
		boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
	},

	// Glass intense
	intense: {
		background: 'rgba(255, 255, 255, 0.1)',
		backdropFilter: 'blur(30px)',
		border: '1px solid rgba(255, 255, 255, 0.2)',
		boxShadow: '0 12px 48px rgba(0, 0, 0, 0.4)'
	},

	// Glass avec accent orange
	accent: {
		background: 'rgba(255, 107, 53, 0.1)',
		backdropFilter: 'blur(20px)',
		border: '1px solid rgba(255, 107, 53, 0.3)',
		boxShadow: '0 8px 32px rgba(255, 107, 53, 0.2)'
	}
};

// ============================================================================
// ANIMATIONS
// ============================================================================

export const animations = {
	// Pulsation de glow
	pulseGlow: {
		animation: 'pulseGlow 2s ease-in-out infinite',
		keyframes: `
			@keyframes pulseGlow {
				0%, 100% {
					box-shadow: 0 0 20px rgba(255, 107, 53, 0.5);
				}
				50% {
					box-shadow: 0 0 40px rgba(255, 107, 53, 0.8);
				}
			}
		`
	},

	// Shimmer holographique
	shimmer: {
		animation: 'shimmer 3s linear infinite',
		keyframes: `
			@keyframes shimmer {
				0% {
					background-position: -200% center;
				}
				100% {
					background-position: 200% center;
				}
			}
		`
	},

	// Rotation de bordure néon
	neonBorder: {
		animation: 'neonBorder 3s linear infinite',
		keyframes: `
			@keyframes neonBorder {
				0% {
					border-color: #ff6b35;
					box-shadow: 0 0 10px rgba(255, 107, 53, 0.5);
				}
				33% {
					border-color: #00d9ff;
					box-shadow: 0 0 10px rgba(0, 217, 255, 0.5);
				}
				66% {
					border-color: #a855f7;
					box-shadow: 0 0 10px rgba(168, 85, 247, 0.5);
				}
				100% {
					border-color: #ff6b35;
					box-shadow: 0 0 10px rgba(255, 107, 53, 0.5);
				}
			}
		`
	},

	// Float animation
	float: {
		animation: 'float 3s ease-in-out infinite',
		keyframes: `
			@keyframes float {
				0%, 100% {
					transform: translateY(0px);
				}
				50% {
					transform: translateY(-10px);
				}
			}
		`
	},

	// Fade in
	fadeIn: {
		animation: 'fadeIn 0.5s ease-out',
		keyframes: `
			@keyframes fadeIn {
				from {
					opacity: 0;
					transform: translateY(20px);
				}
				to {
					opacity: 1;
					transform: translateY(0);
				}
			}
		`
	},

	// Slide in from right
	slideInRight: {
		animation: 'slideInRight 0.5s ease-out',
		keyframes: `
			@keyframes slideInRight {
				from {
					opacity: 0;
					transform: translateX(50px);
				}
				to {
					opacity: 1;
					transform: translateX(0);
				}
			}
		`
	}
};

// ============================================================================
// TYPOGRAPHY
// ============================================================================

export const typography = {
	fonts: {
		display: "'Orbitron', sans-serif",
		body: "'Inter', sans-serif",
		mono: "'JetBrains Mono', monospace"
	},

	sizes: {
		xs: '0.75rem', // 12px
		sm: '0.875rem', // 14px
		base: '1rem', // 16px
		lg: '1.125rem', // 18px
		xl: '1.25rem', // 20px
		'2xl': '1.5rem', // 24px
		'3xl': '1.875rem', // 30px
		'4xl': '2.25rem', // 36px
		'5xl': '3rem', // 48px
		'6xl': '3.75rem', // 60px
		'7xl': '4.5rem' // 72px
	},

	weights: {
		thin: '100',
		light: '300',
		normal: '400',
		medium: '500',
		semibold: '600',
		bold: '700',
		extrabold: '800',
		black: '900'
	}
};

// ============================================================================
// SPACING
// ============================================================================

export const spacing = {
	'0': '0',
	'1': '0.25rem', // 4px
	'2': '0.5rem', // 8px
	'3': '0.75rem', // 12px
	'4': '1rem', // 16px
	'5': '1.25rem', // 20px
	'6': '1.5rem', // 24px
	'8': '2rem', // 32px
	'10': '2.5rem', // 40px
	'12': '3rem', // 48px
	'16': '4rem', // 64px
	'20': '5rem', // 80px
	'24': '6rem' // 96px
};

// ============================================================================
// BORDER RADIUS
// ============================================================================

export const borderRadius = {
	none: '0',
	sm: '0.25rem', // 4px
	base: '0.5rem', // 8px
	md: '0.75rem', // 12px
	lg: '1rem', // 16px
	xl: '1.5rem', // 24px
	'2xl': '2rem', // 32px
	full: '9999px'
};

// ============================================================================
// Z-INDEX
// ============================================================================

export const zIndex = {
	base: 0,
	dropdown: 1000,
	sticky: 1020,
	fixed: 1030,
	modalBackdrop: 1040,
	modal: 1050,
	popover: 1060,
	tooltip: 1070
};

// ============================================================================
// BREAKPOINTS
// ============================================================================

export const breakpoints = {
	sm: '640px',
	md: '768px',
	lg: '1024px',
	xl: '1280px',
	'2xl': '1536px'
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/**
 * Créer un style de glassmorphism personnalisé
 */
export function createGlass(opacity = 0.05, blur = 20, borderOpacity = 0.1) {
	return {
		background: `rgba(255, 255, 255, ${opacity})`,
		backdropFilter: `blur(${blur}px)`,
		border: `1px solid rgba(255, 255, 255, ${borderOpacity})`,
		boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)'
	};
}

/**
 * Créer un glow personnalisé
 */
export function createGlow(r, g, b, intensity = 0.5, size = 20) {
	return `0 0 ${size}px rgba(${r}, ${g}, ${b}, ${intensity})`;
}

/**
 * Créer un gradient personnalisé
 */
export function createGradient(color1, color2, angle = 135) {
	return `linear-gradient(${angle}deg, ${color1} 0%, ${color2} 100%)`;
}

// Export tout le design system
export const theme2040 = {
	colors,
	gradients,
	glows,
	glass,
	animations,
	typography,
	spacing,
	borderRadius,
	zIndex,
	breakpoints,
	createGlass,
	createGlow,
	createGradient
};

export default theme2040;
