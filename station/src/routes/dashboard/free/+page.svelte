<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import SignalCard from '$lib/components/dashboard/SignalCard.svelte';
	import type { Signal } from '$lib/components/dashboard/SignalCard.svelte';

	let signals = $state<Signal[]>([]);
	let loading = $state(true);
	let direction = $state<'all' | 'LONG' | 'SHORT'>('all');

	const mockSignals: Signal[] = [
		{
			ticker: 'AAPL',
			direction: 'LONG',
			score: 78,
			grade: 'B',
			entry: 175.5,
			stop_loss: 170.0,
			take_profit_1: 180.0,
			take_profit_2: 185.0,
			take_profit_3: 190.0,
			risk_reward_ratio: 2.7,
			smc_tags: ['FVG', 'OB'],
			horizon: 'Day',
			timestamp: new Date(Date.now() - 15 * 60 * 1000).toISOString()
		},
		{
			ticker: 'TSLA',
			direction: 'SHORT',
			score: 72,
			grade: 'B',
			entry: 245.0,
			stop_loss: 250.0,
			take_profit_1: 235.0,
			take_profit_2: 230.0,
			take_profit_3: 225.0,
			risk_reward_ratio: 3.0,
			smc_tags: ['CHoCH'],
			horizon: 'Swing',
			timestamp: new Date(Date.now() - 15 * 60 * 1000).toISOString()
		},
		{
			ticker: 'SPY',
			direction: 'LONG',
			score: 65,
			grade: 'C',
			entry: 450.0,
			stop_loss: 445.0,
			take_profit_1: 455.0,
			take_profit_2: 460.0,
			take_profit_3: 465.0,
			risk_reward_ratio: 2.0,
			smc_tags: ['BOS'],
			horizon: 'Scalp',
			timestamp: new Date(Date.now() - 15 * 60 * 1000).toISOString()
		},
		{
			ticker: 'NVDA',
			direction: 'LONG',
			score: 82,
			grade: 'A',
			entry: 720.0,
			stop_loss: 710.0,
			take_profit_1: 735.0,
			take_profit_2: 750.0,
			take_profit_3: 770.0,
			risk_reward_ratio: 3.5,
			smc_tags: ['FVG', 'Liquidity'],
			horizon: 'Day',
			timestamp: new Date(Date.now() - 15 * 60 * 1000).toISOString()
		},
		{
			ticker: 'BTC-USD',
			direction: 'SHORT',
			score: 68,
			grade: 'C',
			entry: 42500,
			stop_loss: 43000,
			take_profit_1: 41500,
			take_profit_2: 40500,
			take_profit_3: 39500,
			risk_reward_ratio: 2.4,
			smc_tags: ['OB'],
			horizon: 'Swing',
			timestamp: new Date(Date.now() - 15 * 60 * 1000).toISOString()
		}
	];

	onMount(() => {
		setTimeout(() => {
			signals = mockSignals.slice(0, 5);
			loading = false;
		}, 800);
	});

	const filteredSignals = $derived(
		direction === 'all' ? signals : signals.filter((s) => s.direction === direction)
	);

	const handleDirectionChange = (e: Event) => {
		direction = (e.target as HTMLSelectElement).value as 'all' | 'LONG' | 'SHORT';
	};
</script>

<div class="free-dashboard">
	<!-- Free Plan Banner -->
	<div class="free-banner">
		<div class="banner-content">
			<div class="banner-icon">🥓</div>
			<div class="banner-text">
				<h2>Free Plan - Limited Features</h2>
				<p>Upgrade to unlock real-time signals, advanced filters, and remove ads</p>
			</div>
		</div>
		<a href="/pricing" class="upgrade-banner-btn"> 🚀 Upgrade Now </a>
	</div>

	<!-- Ad Slot 1 -->
	<div class="ad-slot">
		<div class="ad-placeholder">
			<div class="ad-icon">📢</div>
			<p>Advertisement</p>
			<small>Remove ads with Indicator plan</small>
		</div>
	</div>

	<!-- Delayed Data Warning -->
	<div class="delay-warning">
		<svg
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			<circle cx="12" cy="12" r="10" />
			<polyline points="12 6 12 12 16 14" />
		</svg>
		<span>Signals delayed by 15 minutes</span>
	</div>

	<!-- Filter Section (Limited) -->
	<div class="filter-section">
		<div class="filter-header">
			<h3>Filters (Limited on Free Plan)</h3>
			<a href="/pricing" class="unlock-link">Unlock All Filters →</a>
		</div>
		<div class="filters">
			<div class="filter-item">
				<label for="direction">Direction</label>
				<select id="direction" value={direction} onchange={handleDirectionChange}>
					<option value="all">Both</option>
					<option value="LONG">Long Only</option>
					<option value="SHORT">Short Only</option>
				</select>
			</div>
			<div class="filter-item locked">
				<label>Horizon <span class="lock-icon">🔒</span>
					<select disabled>
						<option>Unlock with Indicator plan</option>
					</select>
				</label>
			</div>
			<div class="filter-item locked">
				<label>Min Score <span class="lock-icon">🔒</span>
					<input type="range" disabled />
				</label>
			</div>
		</div>
	</div>

	<!-- Ad Slot 2 -->
	<div class="ad-slot horizontal">
		<div class="ad-placeholder">
			<div class="ad-icon">📢</div>
			<p>Sponsored Content</p>
		</div>
	</div>

	<!-- Signals Grid -->
	<div class="signals-section">
		<div class="section-header">
			<h3>Latest Signals (Max 5)</h3>
			<div class="delayed-badge">15 min delayed</div>
		</div>

		{#if loading}
			<div class="loading">
				<div class="spinner"></div>
				<p>Loading signals...</p>
			</div>
		{:else if filteredSignals.length === 0}
			<div class="empty-state">
				<div class="empty-icon">📊</div>
				<h4>No signals available</h4>
				<p>Try adjusting your filters or check back later</p>
			</div>
		{:else}
			<div class="signals-grid">
				{#each filteredSignals as signal}
					<SignalCard {signal} />
				{/each}
			</div>
		{/if}
	</div>

	<!-- Ad Slot 3 -->
	<div class="ad-slot">
		<div class="ad-placeholder">
			<div class="ad-icon">📢</div>
			<p>Advertisement</p>
		</div>
	</div>

	<!-- Upgrade CTA -->
	<div class="upgrade-cta-section">
		<div class="cta-content">
			<div class="cta-icon">🚀</div>
			<h3>Unlock Real-Time Signals & Advanced Features</h3>
			<p>Get instant alerts, unlimited signals, advanced filters, and remove all ads</p>
			<div class="cta-features">
				<div class="feature">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Real-time signals
				</div>
				<div class="feature">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Unlimited watchlist
				</div>
				<div class="feature">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 6 9 17 4 12" />
					</svg>
					No ads
				</div>
				<div class="feature">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Advanced filters
				</div>
			</div>
			<a href="/pricing" class="cta-button">View Pricing Plans</a>
		</div>
	</div>
</div>

<style>
	.free-dashboard {
		max-width: 1400px;
		margin: 0 auto;
	}

	.free-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.5rem;
		background: linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(220, 38, 38, 0.1) 100%);
		border: 2px solid rgba(239, 68, 68, 0.3);
		border-radius: 1rem;
		margin-bottom: 1.5rem;
	}

	.banner-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.banner-icon {
		font-size: 2.5rem;
	}

	.banner-text h2 {
		margin: 0 0 0.25rem 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.banner-text p {
		margin: 0;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.upgrade-banner-btn {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		border-radius: 0.5rem;
		color: #1f2937;
		font-weight: 700;
		text-decoration: none;
		white-space: nowrap;
		transition: all 0.3s ease;
	}

	.upgrade-banner-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(255, 107, 53, 0.4);
	}

	.ad-slot {
		margin-bottom: 1.5rem;
	}

	.ad-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: rgba(255, 255, 255, 0.02);
		border: 2px dashed rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		text-align: center;
	}

	.ad-slot.horizontal .ad-placeholder {
		flex-direction: row;
		gap: 1rem;
		padding: 1.5rem;
	}

	.ad-icon {
		font-size: 2rem;
		opacity: 0.5;
	}

	.ad-placeholder p {
		margin: 0.5rem 0 0 0;
		color: #6b7280;
		font-size: 1rem;
	}

	.ad-placeholder small {
		color: #4b5563;
		font-size: 0.75rem;
	}

	.delay-warning {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 0.5rem;
		color: #fbbf24;
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
	}

	.filter-section {
		padding: 1.5rem;
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		margin-bottom: 1.5rem;
	}

	.filter-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.filter-header h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.unlock-link {
		color: #ff6b35;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.unlock-link:hover {
		color: #fbbf24;
	}

	.filters {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.filter-item label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #e5e7eb;
	}

	.filter-item select,
	.filter-item input {
		width: 100%;
		padding: 0.625rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #f9fafb;
		font-size: 0.875rem;
	}

	.filter-item.locked {
		opacity: 0.4;
	}

	.lock-icon {
		margin-left: 0.25rem;
	}

	.signals-section {
		margin-bottom: 1.5rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.5rem;
	}

	.section-header h3 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.delayed-badge {
		padding: 0.375rem 0.75rem;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 9999px;
		color: #fbbf24;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		color: #9ca3af;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(255, 107, 53, 0.2);
		border-top-color: #ff6b35;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
	}

	.empty-icon {
		font-size: 4rem;
		margin-bottom: 1rem;
		opacity: 0.5;
	}

	.empty-state h4 {
		margin: 0 0 0.5rem 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.empty-state p {
		margin: 0;
		color: #9ca3af;
	}

	.signals-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: 1.5rem;
	}

	.upgrade-cta-section {
		padding: 3rem;
		background: linear-gradient(135deg, rgba(168, 85, 247, 0.1) 0%, rgba(236, 72, 153, 0.1) 100%);
		border: 2px solid rgba(168, 85, 247, 0.3);
		border-radius: 1rem;
		text-align: center;
	}

	.cta-icon {
		font-size: 4rem;
		margin-bottom: 1rem;
	}

	.cta-content h3 {
		margin: 0 0 0.75rem 0;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.cta-content p {
		margin: 0 0 1.5rem 0;
		color: #9ca3af;
		font-size: 1rem;
	}

	.cta-features {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		max-width: 800px;
		margin: 0 auto 2rem;
	}

	.feature {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #10b981;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.cta-button {
		display: inline-block;
		padding: 1rem 2rem;
		background: linear-gradient(135deg, #a855f7 0%, #ec4899 100%);
		border-radius: 0.5rem;
		color: #fff;
		font-size: 1rem;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(168, 85, 247, 0.4);
	}

	@media (max-width: 640px) {
		.free-banner {
			flex-direction: column;
			gap: 1rem;
			text-align: center;
		}

		.banner-content {
			flex-direction: column;
		}

		.signals-grid {
			grid-template-columns: 1fr;
		}

		.cta-features {
			grid-template-columns: 1fr;
		}
	}
</style>
