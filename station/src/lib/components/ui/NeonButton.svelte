<script lang="ts">
	export let variant: 'primary' | 'secondary' | 'success' | 'danger' = 'primary';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let loading: boolean = false;
	export let disabled: boolean = false;
	export let className: string = '';
</script>

<button
	class="neon-button {variant} {size} {className}"
	{disabled}
	on:click
>
	{#if loading}
		<span class="spinner"></span>
	{/if}
	<slot />
</button>

<style>
	.neon-button {
		font-family: 'Inter', sans-serif;
		font-weight: 600;
		border-radius: 0.5rem;
		transition: all 0.3s ease;
		border: none;
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		position: relative;
		overflow: hidden;
	}

	.neon-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.neon-button.sm {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.neon-button.md {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
	}

	.neon-button.lg {
		padding: 1rem 2rem;
		font-size: 1.125rem;
	}

	.neon-button.primary {
		background: linear-gradient(to right, #ff6b35, #f97316);
		color: white;
	}

	.neon-button.primary:hover:not(:disabled) {
		box-shadow: 0 0 25px rgba(255, 107, 53, 0.6), 0 0 50px rgba(255, 107, 53, 0.3);
		transform: translateY(-2px);
	}

	.neon-button.secondary {
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(16px);
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.neon-button.secondary:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.15);
		border-color: rgba(255, 107, 53, 0.5);
		box-shadow: 0 0 15px rgba(255, 107, 53, 0.3);
	}

	.neon-button.success {
		background: linear-gradient(to right, #10b981, #059669);
		color: white;
	}

	.neon-button.success:hover:not(:disabled) {
		box-shadow: 0 0 25px rgba(16, 185, 129, 0.6);
		transform: translateY(-2px);
	}

	.neon-button.danger {
		background: linear-gradient(to right, #ef4444, #dc2626);
		color: white;
	}

	.neon-button.danger:hover:not(:disabled) {
		box-shadow: 0 0 25px rgba(239, 68, 68, 0.6);
		transform: translateY(-2px);
	}

	.spinner {
		display: inline-block;
		width: 1em;
		height: 1em;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
