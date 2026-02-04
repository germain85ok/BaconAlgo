<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { BITCOIN_ADDRESS, ETHEREUM_ADDRESS } from '$lib/config/env';

	const dispatch = createEventDispatcher();

	interface PaymentMethod {
		name: string;
		icon: string;
		address?: string;
		color: string;
	}

	const paymentMethods: PaymentMethod[] = [
		{
			name: 'PayPal',
			icon: '💳',
			color: '#0070ba'
		},
		{
			name: 'Stripe',
			icon: '💰',
			color: '#635bff'
		},
		{
			name: 'Bitcoin',
			icon: '₿',
			address: BITCOIN_ADDRESS || 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
			color: '#f7931a'
		},
		{
			name: 'Ethereum',
			icon: '⟠',
			address: ETHEREUM_ADDRESS || '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb1',
			color: '#627eea'
		}
	];

	let showQR: string | null = null;
	let copiedAddress: string | null = null;

	function toggleQR(name: string) {
		showQR = showQR === name ? null : name;
	}

	function copyToClipboard(address: string, name: string) {
		navigator.clipboard.writeText(address);
		copiedAddress = name;
		setTimeout(() => {
			copiedAddress = null;
		}, 2000);
	}

	function handleClick(method: PaymentMethod) {
		if (method.address) {
			toggleQR(method.name);
		} else {
			dispatch('donate', { method: method.name });
		}
	}
</script>

<div class="donation-links">
	<h3>Support BaconAlgo</h3>
	<p class="subtitle">Choose your payment method</p>

	<div class="methods-grid">
		{#each paymentMethods as method (method.name)}
			<button
				class="payment-button"
				style="--method-color: {method.color}"
				on:click={() => handleClick(method)}
			>
				<span class="icon">{method.icon}</span>
				<span class="name">{method.name}</span>
				{#if method.address}
					<span class="has-qr">📱</span>
				{/if}
			</button>

			{#if showQR === method.name && method.address}
				<div class="qr-popup">
					<div class="qr-header">
						<h4>{method.name} Address</h4>
						<button class="close-btn" on:click={() => (showQR = null)}>✕</button>
					</div>
					<div class="address-box">
						<code>{method.address}</code>
						<button
							class="copy-btn"
							on:click={() => copyToClipboard(method.address!, method.name)}
						>
							{copiedAddress === method.name ? '✓ Copied!' : '📋 Copy'}
						</button>
					</div>
					<div class="qr-note">
						Scan QR code or copy address to send {method.name}
					</div>
				</div>
			{/if}
		{/each}
	</div>
</div>

<style>
	.donation-links {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 350px;
	}

	h3 {
		margin: 0 0 0.5rem 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		text-align: center;
	}

	.subtitle {
		margin: 0 0 1.5rem 0;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		text-align: center;
	}

	.methods-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		position: relative;
	}

	.payment-button {
		background: rgba(255, 255, 255, 0.05);
		border: 2px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		transition: all 0.3s ease;
		position: relative;
		overflow: hidden;
	}

	.payment-button::before {
		content: '';
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(
			90deg,
			transparent,
			var(--method-color, rgba(255, 255, 255, 0.1)),
			transparent
		);
		opacity: 0;
		transition: all 0.5s;
	}

	.payment-button:hover {
		border-color: var(--method-color, rgba(255, 255, 255, 0.3));
		transform: translateY(-4px);
		box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
	}

	.payment-button:hover::before {
		left: 100%;
		opacity: 0.3;
	}

	.icon {
		font-size: 2rem;
	}

	.name {
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.has-qr {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		font-size: 0.75rem;
		opacity: 0.6;
	}

	.qr-popup {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: rgba(20, 20, 30, 0.98);
		backdrop-filter: blur(20px);
		border: 2px solid rgba(249, 115, 22, 0.5);
		border-radius: 12px;
		padding: 1.5rem;
		z-index: 100;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
		width: 90%;
		max-width: 400px;
	}

	.qr-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.qr-header h4 {
		margin: 0;
		font-size: 1rem;
		color: rgba(255, 255, 255, 0.9);
	}

	.close-btn {
		background: none;
		border: none;
		color: rgba(255, 255, 255, 0.6);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.close-btn:hover {
		color: rgba(255, 255, 255, 0.9);
	}

	.address-box {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1rem;
	}

	.address-box code {
		display: block;
		font-family: 'Courier New', monospace;
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.9);
		word-break: break-all;
		margin-bottom: 0.75rem;
	}

	.copy-btn {
		background: #f97316;
		border: none;
		border-radius: 6px;
		padding: 0.5rem 1rem;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		width: 100%;
		transition: all 0.3s ease;
	}

	.copy-btn:hover {
		background: #ff6b35;
		transform: translateY(-2px);
	}

	.qr-note {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		text-align: center;
	}
</style>
