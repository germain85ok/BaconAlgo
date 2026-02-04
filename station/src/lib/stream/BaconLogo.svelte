<script lang="ts">
	import { onMount } from 'svelte';

	let animate = false;

	onMount(() => {
		animate = true;
	});
</script>

<div class="bacon-logo" class:animate>
	<svg
		width="200"
		height="80"
		viewBox="0 0 200 80"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
		class="logo-svg"
	>
		<!-- Bacon strips -->
		<g class="bacon-strips">
			<!-- Strip 1 -->
			<path
				d="M10 25 Q20 15 30 25 T50 25"
				stroke="url(#bacon-gradient1)"
				stroke-width="8"
				fill="none"
				stroke-linecap="round"
				class="strip strip-1"
			/>
			<!-- Strip 2 -->
			<path
				d="M10 40 Q20 30 30 40 T50 40"
				stroke="url(#bacon-gradient2)"
				stroke-width="8"
				fill="none"
				stroke-linecap="round"
				class="strip strip-2"
			/>
			<!-- Strip 3 -->
			<path
				d="M10 55 Q20 45 30 55 T50 55"
				stroke="url(#bacon-gradient3)"
				stroke-width="8"
				fill="none"
				stroke-linecap="round"
				class="strip strip-3"
			/>
		</g>

		<!-- Gradients -->
		<defs>
			<linearGradient id="bacon-gradient1" x1="0%" y1="0%" x2="100%" y2="0%">
				<stop offset="0%" style="stop-color:#F97316;stop-opacity:1" />
				<stop offset="50%" style="stop-color:#FFD700;stop-opacity:1" />
				<stop offset="100%" style="stop-color:#FF6B35;stop-opacity:1" />
			</linearGradient>
			<linearGradient id="bacon-gradient2" x1="0%" y1="0%" x2="100%" y2="0%">
				<stop offset="0%" style="stop-color:#FF6B35;stop-opacity:1" />
				<stop offset="50%" style="stop-color:#F97316;stop-opacity:1" />
				<stop offset="100%" style="stop-color:#FFD700;stop-opacity:1" />
			</linearGradient>
			<linearGradient id="bacon-gradient3" x1="0%" y1="0%" x2="100%" y2="0%">
				<stop offset="0%" style="stop-color:#FFD700;stop-opacity:1" />
				<stop offset="50%" style="stop-color:#FF6B35;stop-opacity:1" />
				<stop offset="100%" style="stop-color:#F97316;stop-opacity:1" />
			</linearGradient>
			<linearGradient id="text-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
				<stop offset="0%" style="stop-color:#F97316;stop-opacity:1" />
				<stop offset="100%" style="stop-color:#FFD700;stop-opacity:1" />
			</linearGradient>
		</defs>

		<!-- Text -->
		<text
			x="65"
			y="35"
			font-family="Arial, sans-serif"
			font-size="24"
			font-weight="bold"
			fill="url(#text-gradient)"
			class="logo-text"
		>
			BaconAlgo
		</text>
		<text
			x="65"
			y="55"
			font-family="Arial, sans-serif"
			font-size="12"
			fill="rgba(255, 255, 255, 0.7)"
			class="logo-subtext"
		>
			SIZZLING TRADES
		</text>
	</svg>

	<div class="glow-effect" />
</div>

<style>
	.bacon-logo {
		position: relative;
		display: inline-block;
		filter: drop-shadow(0 0 20px rgba(249, 115, 22, 0.5));
	}

	.logo-svg {
		position: relative;
		z-index: 2;
	}

	.strip {
		stroke-dasharray: 50;
		stroke-dashoffset: 50;
		animation: draw 2s ease-in-out forwards, sizzle 3s ease-in-out infinite;
	}

	.strip-1 {
		animation-delay: 0s, 2s;
	}

	.strip-2 {
		animation-delay: 0.3s, 2.3s;
	}

	.strip-3 {
		animation-delay: 0.6s, 2.6s;
	}

	@keyframes draw {
		to {
			stroke-dashoffset: 0;
		}
	}

	@keyframes sizzle {
		0%,
		100% {
			transform: translateY(0);
			opacity: 1;
		}
		25% {
			transform: translateY(-2px);
			opacity: 0.9;
		}
		50% {
			transform: translateY(0);
			opacity: 1;
		}
		75% {
			transform: translateY(2px);
			opacity: 0.9;
		}
	}

	.logo-text {
		opacity: 0;
		animation: fadeIn 1s ease-in-out 1s forwards;
	}

	.logo-subtext {
		opacity: 0;
		animation: fadeIn 1s ease-in-out 1.5s forwards;
		letter-spacing: 0.15em;
	}

	@keyframes fadeIn {
		to {
			opacity: 1;
		}
	}

	.glow-effect {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 120%;
		height: 120%;
		background: radial-gradient(
			circle,
			rgba(249, 115, 22, 0.3) 0%,
			rgba(255, 215, 0, 0.2) 30%,
			transparent 70%
		);
		animation: pulse-glow 3s ease-in-out infinite;
		z-index: 1;
		pointer-events: none;
	}

	@keyframes pulse-glow {
		0%,
		100% {
			opacity: 0.5;
			transform: translate(-50%, -50%) scale(1);
		}
		50% {
			opacity: 0.8;
			transform: translate(-50%, -50%) scale(1.1);
		}
	}

	.bacon-logo.animate .strip {
		animation-play-state: running;
	}
</style>
