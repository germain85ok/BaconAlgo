/**
 * BaconAlgo 2040 - Neon Button Component
 * Bouton avec bordure néon et effet de glow
 */

<script lang="ts">
	interface Props {
		variant?: 'primary' | 'secondary' | 'success' | 'danger';
		size?: 'sm' | 'md' | 'lg';
		glow?: boolean;
		disabled?: boolean;
		loading?: boolean;
		type?: 'button' | 'submit' | 'reset';
		onclick?: () => void;
		children?: import('svelte').Snippet;
		class?: string;
	}

	let {
		variant = 'primary',
		size = 'md',
		glow = true,
		disabled = false,
		loading = false,
		type = 'button',
		onclick,
		children,
		class: className = ''
	}: Props = $props();

	const variantClasses = {
		primary: 'bg-gradient-primary text-white shadow-glow-md hover:shadow-glow-lg',
		secondary: 'bg-white/10 text-white border-bacon-orange/50 border-2 hover:bg-bacon-orange/20',
		success: 'bg-gradient-to-r from-green-500 to-green-600 text-white shadow-glow-green-sm',
		danger: 'bg-gradient-to-r from-red-500 to-red-600 text-white shadow-glow-red-sm'
	};

	const sizeClasses = {
		sm: 'px-3 py-1.5 text-sm',
		md: 'px-6 py-3 text-base',
		lg: 'px-8 py-4 text-lg'
	};
</script>

<button
	{type}
	{disabled}
	onclick={onclick}
	class="neon-button {variantClasses[variant]} {sizeClasses[size]} {glow && variant !== 'secondary' ? 'animate-pulse-glow' : ''} {className}"
>
	{#if loading}
		<div class="loading-spinner"></div>
	{/if}
	{#if children}
		{@render children()}
	{/if}
</button>

<style>
	.neon-button {
		font-weight: 600;
		border-radius: 0.75rem;
		transition: all 0.3s ease;
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		position: relative;
		overflow: hidden;
	}

	.neon-button:hover:not(:disabled) {
		transform: translateY(-2px);
	}

	.neon-button:active:not(:disabled) {
		transform: translateY(0);
	}

	.neon-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.loading-spinner {
		width: 1rem;
		height: 1rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Effet de shimmer au hover */
	.neon-button::before {
		content: '';
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(
			90deg,
			transparent,
			rgba(255, 255, 255, 0.3),
			transparent
		);
		transition: left 0.5s;
	}

	.neon-button:hover::before {
		left: 100%;
	}
</style>
