/**
 * BaconAlgo 2040 - Glass Panel Component
 * Panneau glassmorphism avec bordure néon et effets holographiques
 */

<script lang="ts">
	interface Props {
		variant?: 'default' | 'accent' | 'intense';
		glow?: boolean;
		animated?: boolean;
		children?: import('svelte').Snippet;
		class?: string;
	}

	let {
		variant = 'default',
		glow = false,
		animated = false,
		children,
		class: className = ''
	}: Props = $props();

	const variantClasses = {
		default: 'bg-white/5 border-white/10',
		accent: 'bg-bacon-orange/10 border-bacon-orange/30',
		intense: 'bg-white/10 border-white/20'
	};

	const glowClasses = {
		default: 'shadow-glass',
		accent: 'shadow-glow-md',
		intense: 'shadow-glass-lg'
	};
</script>

<div
	class="glass-panel {variantClasses[variant]} {glow ? glowClasses[variant] : 'shadow-glass'} {animated ? 'animate-fade-in' : ''} {className}"
>
	{#if children}
		{@render children()}
	{/if}
</div>

<style>
	.glass-panel {
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-radius: 1rem;
		border-width: 1px;
		transition: all 0.3s ease;
	}

	.glass-panel:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 107, 53, 0.4);
		transform: translateY(-2px);
	}
</style>
