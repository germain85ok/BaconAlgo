<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { page } from '$app/stores';
	
	// Dashboard
	import StreamDashboard from '$lib/stream/StreamDashboard.svelte';
	
	// Widgets
	import MarketCountdown from '$lib/stream/widgets/MarketCountdown.svelte';
	import FearGreedIndex from '$lib/stream/widgets/FearGreedIndex.svelte';
	import VixMeter from '$lib/stream/widgets/VixMeter.svelte';
	import RocketSignals from '$lib/stream/widgets/RocketSignals.svelte';
	import MarketHeatmap from '$lib/stream/widgets/MarketHeatmap.svelte';
	import CryptoTicker from '$lib/stream/widgets/CryptoTicker.svelte';
	import NewsHeadlines from '$lib/stream/widgets/NewsHeadlines.svelte';
	import TopMovers from '$lib/stream/widgets/TopMovers.svelte';
	import LiveClock from '$lib/stream/widgets/LiveClock.svelte';
	import LiveViewerCount from '$lib/stream/widgets/LiveViewerCount.svelte';
	import MusicVisualizer from '$lib/stream/widgets/MusicVisualizer.svelte';
	
	// Donations
	import DonationAlert from '$lib/stream/donations/DonationAlert.svelte';
	import DonationFeed from '$lib/stream/donations/DonationFeed.svelte';
	import DonationTiers from '$lib/stream/donations/DonationTiers.svelte';
	import DonationGoalTracker from '$lib/stream/donations/DonationGoalTracker.svelte';
	import DonorLeaderboard from '$lib/stream/donations/DonorLeaderboard.svelte';
	import DonationLinks from '$lib/stream/donations/DonationLinks.svelte';
	
	// Social & Sponsors
	import SponsorBanner from '$lib/stream/sponsors/SponsorBanner.svelte';
	import SocialLinks from '$lib/stream/social/SocialLinks.svelte';
	import ChatOverlay from '$lib/stream/social/ChatOverlay.svelte';
	import StreamSchedule from '$lib/stream/social/StreamSchedule.svelte';
	
	// Music
	import SongRequestQueue from '$lib/stream/music/SongRequestQueue.svelte';
	
	// Logo
	import BaconLogo from '$lib/stream/BaconLogo.svelte';
	
	// Stores
	import { initSignalsStore, cleanupSignalsStore } from '$lib/stores/signals';
	import { initDonationsStore, cleanupDonationsStore } from '$lib/stores/donations';
	import { initMarketStore } from '$lib/stores/market';
	
	export let data;
	
	$: widget = $page.url.searchParams.get('widget') || data.widget || 'dashboard';
	
	// Valid widget names for validation
	const validWidgets = [
		'dashboard', 'countdown', 'feargreed', 'vix', 'signals', 'heatmap',
		'crypto', 'news', 'movers', 'clock', 'viewers', 'visualizer',
		'donations', 'donation-feed', 'donation-tiers', 'donation-goal',
		'leaderboard', 'donation-links', 'sponsor', 'social', 'chat',
		'schedule', 'songs', 'logo'
	];
	
	$: isValidWidget = validWidgets.includes(widget);
	
	onMount(async () => {
		try {
			// Initialize all stores that might be needed
			await initSignalsStore();
			await initDonationsStore();
			initMarketStore();
		} catch (error) {
			console.error('Failed to initialize stores:', error);
		}
	});
	
	onDestroy(() => {
		cleanupSignalsStore();
		cleanupDonationsStore();
	});
</script>

<svelte:head>
	<title>{data.meta.title}</title>
	<meta name="description" content={data.meta.description} />
</svelte:head>

<div class="stream-container" class:full-dashboard={widget === 'dashboard'}>
	{#if !isValidWidget}
		<div class="error-message">
			<h1>Invalid Widget</h1>
			<p>Widget "{widget}" not found. Valid options:</p>
			<ul>
				{#each validWidgets as validWidget}
					<li>{validWidget}</li>
				{/each}
			</ul>
		</div>
	{:else if widget === 'dashboard'}
		<StreamDashboard />
	{:else if widget === 'countdown'}
		<MarketCountdown />
	{:else if widget === 'feargreed'}
		<FearGreedIndex />
	{:else if widget === 'vix'}
		<VixMeter />
	{:else if widget === 'signals'}
		<RocketSignals />
	{:else if widget === 'heatmap'}
		<MarketHeatmap />
	{:else if widget === 'crypto'}
		<CryptoTicker />
	{:else if widget === 'news'}
		<NewsHeadlines />
	{:else if widget === 'movers'}
		<TopMovers />
	{:else if widget === 'clock'}
		<LiveClock />
	{:else if widget === 'viewers'}
		<LiveViewerCount />
	{:else if widget === 'visualizer'}
		<MusicVisualizer />
	{:else if widget === 'donations'}
		<DonationAlert />
	{:else if widget === 'donation-feed'}
		<DonationFeed />
	{:else if widget === 'donation-tiers'}
		<DonationTiers />
	{:else if widget === 'donation-goal'}
		<DonationGoalTracker />
	{:else if widget === 'leaderboard'}
		<DonorLeaderboard />
	{:else if widget === 'donation-links'}
		<DonationLinks />
	{:else if widget === 'sponsor'}
		<SponsorBanner />
	{:else if widget === 'social'}
		<SocialLinks />
	{:else if widget === 'chat'}
		<ChatOverlay />
	{:else if widget === 'schedule'}
		<StreamSchedule />
	{:else if widget === 'songs'}
		<SongRequestQueue />
	{:else if widget === 'logo'}
		<BaconLogo />
	{/if}
</div>

<style>
	.stream-container {
		/* Transparent background for OBS chroma key */
		background: transparent;
		margin: 0;
		padding: 0;
		overflow: hidden;
		font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}
	
	/* Full viewport for dashboard */
	.full-dashboard {
		position: fixed;
		top: 0;
		left: 0;
		width: 1920px;
		height: 1080px;
	}
	
	/* Error message styling */
	.error-message {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: rgba(255, 0, 0, 0.1);
		border: 2px solid rgba(255, 0, 0, 0.5);
		border-radius: 12px;
		padding: 2rem;
		color: white;
		max-width: 600px;
	}
	
	.error-message h1 {
		margin: 0 0 1rem 0;
		color: #ff6b6b;
	}
	
	.error-message p {
		margin: 0 0 1rem 0;
	}
	
	.error-message ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.5rem;
	}
	
	.error-message li {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.5rem;
		border-radius: 4px;
		font-size: 0.875rem;
	}
</style>
