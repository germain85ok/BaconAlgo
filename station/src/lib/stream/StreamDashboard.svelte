<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import MarketCountdown from './widgets/MarketCountdown.svelte';
	import FearGreedIndex from './widgets/FearGreedIndex.svelte';
	import RocketSignals from './widgets/RocketSignals.svelte';
	import DonationAlert from './donations/DonationAlert.svelte';
	import DonationTiers from './donations/DonationTiers.svelte';
	import DonationGoalTracker from './donations/DonationGoalTracker.svelte';
	import SocialLinks from './social/SocialLinks.svelte';

	import { initSignalsStore, cleanupSignalsStore } from '$lib/stores/signals';
	import { initDonationsStore, cleanupDonationsStore } from '$lib/stores/donations';
	import { initMarketStore } from '$lib/stores/market';
	import { settings } from '$lib/stores/settings';

	onMount(async () => {
		// Initialize all stores
		await initSignalsStore();
		await initDonationsStore();
		initMarketStore();
	});

	onDestroy(() => {
		// Cleanup subscriptions
		cleanupSignalsStore();
		cleanupDonationsStore();
	});
</script>

<div class="stream-dashboard">
	<!-- Top Bar: Market Countdown & Fear/Greed -->
	<div class="top-bar">
		<MarketCountdown />
		<FearGreedIndex />
	</div>

	<!-- Main Content Area -->
	<div class="main-content">
		<!-- Left Side: Hot Signals -->
		<div class="left-panel">
			{#if $settings.showSignals}
				<RocketSignals />
			{/if}
		</div>

		<!-- Right Side: Donation Info -->
		<div class="right-panel">
			{#if $settings.showDonations}
				<DonationGoalTracker />
				<DonationTiers />
			{/if}
		</div>
	</div>

	<!-- Bottom Bar: Social Links -->
	<div class="bottom-bar">
		<SocialLinks />
	</div>

	<!-- Floating Donation Alerts -->
	{#if $settings.showDonations}
		<DonationAlert />
	{/if}
</div>

<style>
	.stream-dashboard {
		position: fixed;
		top: 0;
		left: 0;
		width: 1920px;
		height: 1080px;
		background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 100%);
		overflow: hidden;
		font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.top-bar {
		position: absolute;
		top: 20px;
		left: 20px;
		right: 20px;
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 20px;
		z-index: 10;
	}

	.main-content {
		position: absolute;
		top: 140px;
		left: 20px;
		right: 20px;
		bottom: 100px;
		display: flex;
		justify-content: space-between;
		gap: 20px;
	}

	.left-panel {
		flex: 0 0 600px;
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.right-panel {
		flex: 0 0 400px;
		display: flex;
		flex-direction: column;
		gap: 20px;
		align-items: flex-end;
	}

	.bottom-bar {
		position: absolute;
		bottom: 20px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 10;
	}

	/* Glass morphism base styles */
	:global(.glass-card) {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
	}

	:global(.glass-button) {
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		transition: all 0.3s ease;
		cursor: pointer;
	}

	:global(.glass-button:hover) {
		background: rgba(255, 255, 255, 0.15);
		border-color: rgba(255, 255, 255, 0.3);
		transform: translateY(-2px);
		box-shadow: 0 4px 20px rgba(255, 107, 53, 0.3);
	}

	/* Brand colors */
	:global(:root) {
		--brand-primary: #ff6b35;
		--brand-accent: #f7931e;
		--grade-s: #ffd700;
		--grade-a: #ff6b35;
		--grade-b: #4a90e2;
		--grade-c: #95a5a6;
		--buy-color: #2ecc71;
		--sell-color: #e74c3c;
	}
</style>
