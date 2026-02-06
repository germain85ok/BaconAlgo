<script lang="ts">
	import { onMount } from 'svelte';
	import { marketDataService, type MarketSummary } from '$lib/services/marketData';

	let currentTime = $state(new Date());
	let marketData = $state<MarketSummary | null>(null);
	let language = $state<'fr' | 'en'>('fr');
	let countdown = $state('');

	// Labels
	const labels = {
		fr: {
			title: 'BaconAlgo 2030 - Station de Trading',
			marketOpen: 'Ouverture du marché dans:',
			indices: 'Indices',
			crypto: 'Crypto',
			signals: 'Signaux récents',
			donate: 'Faire un don',
			support: 'Supportez-nous'
		},
		en: {
			title: 'BaconAlgo 2030 - Trading Station',
			marketOpen: 'Market opens in:',
			indices: 'Indices',
			crypto: 'Crypto',
			signals: 'Recent Signals',
			donate: 'Donate',
			support: 'Support Us'
		}
	};

	const t = $derived(labels[language]);

	onMount(() => {
		// Update time every second
		const timeInterval = setInterval(() => {
			currentTime = new Date();
			updateCountdown();
		}, 1000);

		// Fetch market data
		marketDataService.fetchMarketSummary().then(data => {
			marketData = data;
		});

		// Auto-update market data
		marketDataService.startAutoUpdate(data => {
			marketData = data;
		}, 10000);

		// Auto-switch language every 10 seconds
		const langInterval = setInterval(() => {
			language = language === 'fr' ? 'en' : 'fr';
		}, 10000);

		return () => {
			clearInterval(timeInterval);
			clearInterval(langInterval);
			marketDataService.stopAutoUpdate();
		};
	});

	function updateCountdown() {
		const now = new Date();
		const marketOpen = new Date();
		marketOpen.setHours(9, 30, 0, 0);

		if (now > marketOpen) {
			marketOpen.setDate(marketOpen.getDate() + 1);
		}

		const diff = marketOpen.getTime() - now.getTime();
		const hours = Math.floor(diff / (1000 * 60 * 60));
		const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
		const seconds = Math.floor((diff % (1000 * 60)) / 1000);

		countdown = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
	}
</script>

<div class="stream-overlay">
	<!-- Header -->
	<header class="overlay-header">
		<div class="logo">
			<div class="logo-icon">🥓</div>
			<div class="logo-text">{t.title}</div>
		</div>
		<div class="time">{currentTime.toLocaleTimeString()}</div>
	</header>

	<!-- Main Content -->
	<div class="overlay-content">
		<!-- Left Column - Market Data -->
		<div class="column left">
			<div class="section">
				<h2 class="section-title">{t.indices}</h2>
				{#if marketData}
					<div class="data-list">
						{#each marketData.indices as index}
							<div class="data-item">
								<span class="symbol">{index.symbol}</span>
								<span class="price">${index.price.toFixed(2)}</span>
								<span class="change" class:positive={index.changePercent > 0} class:negative={index.changePercent < 0}>
									{index.changePercent > 0 ? '+' : ''}{index.changePercent.toFixed(2)}%
								</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<div class="section">
				<h2 class="section-title">{t.crypto}</h2>
				{#if marketData}
					<div class="data-list">
						{#each marketData.crypto.slice(0, 5) as crypto}
							<div class="data-item">
								<span class="symbol">{crypto.symbol}</span>
								<span class="price">${crypto.price.toLocaleString(undefined, {maximumFractionDigits: 0})}</span>
								<span class="change" class:positive={crypto.changePercent24h > 0} class:negative={crypto.changePercent24h < 0}>
									{crypto.changePercent24h > 0 ? '+' : ''}{crypto.changePercent24h.toFixed(2)}%
								</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<!-- Center Column - Countdown -->
		<div class="column center">
			<div class="countdown-section">
				<div class="countdown-label">{t.marketOpen}</div>
				<div class="countdown-display">{countdown}</div>
			</div>
		</div>

		<!-- Right Column - Signals -->
		<div class="column right">
			<div class="section">
				<h2 class="section-title">{t.signals}</h2>
				<div class="signals-list">
					<div class="signal-item">
						<div class="signal-header">
							<span class="signal-symbol">NVDA</span>
							<span class="signal-badge long">LONG</span>
						</div>
						<div class="signal-score">85/100</div>
						<div class="signal-entry">Entry: $870</div>
					</div>

					<div class="signal-item">
						<div class="signal-header">
							<span class="signal-symbol">TSLA</span>
							<span class="signal-badge long">LONG</span>
						</div>
						<div class="signal-score">72/100</div>
						<div class="signal-entry">Entry: $245</div>
					</div>

					<div class="signal-item">
						<div class="signal-header">
							<span class="signal-symbol">SPY</span>
							<span class="signal-badge short">SHORT</span>
						</div>
						<div class="signal-score">68/100</div>
						<div class="signal-entry">Entry: $450</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Footer - Donation Buttons -->
	<footer class="overlay-footer">
		<div class="donation-section">
			<span class="donation-label">{t.support}:</span>
			<div class="donation-buttons">
				<button class="donate-btn">💳 {t.donate} $5</button>
				<button class="donate-btn">💳 {t.donate} $10</button>
				<button class="donate-btn">💳 {t.donate} $20</button>
			</div>
		</div>
	</footer>
</div>

<style>
	.stream-overlay {
		width: 1920px;
		height: 1080px;
		background: linear-gradient(135deg, #0a0a0f 0%, #1a1f3a 100%);
		color: #f9fafb;
		display: flex;
		flex-direction: column;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		overflow: hidden;
	}

	.overlay-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem 3rem;
		background: rgba(255, 255, 255, 0.05);
		border-bottom: 2px solid rgba(255, 107, 53, 0.3);
	}

	.logo {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.logo-icon {
		font-size: 3rem;
	}

	.logo-text {
		font-size: 2rem;
		font-weight: 700;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.time {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fbbf24;
	}

	.overlay-content {
		flex: 1;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 2rem;
		padding: 3rem;
	}

	.column {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.section {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 2rem;
	}

	.section-title {
		font-size: 1.75rem;
		font-weight: 700;
		margin-bottom: 1.5rem;
		color: #ff6b35;
	}

	.data-list {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.data-item {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		align-items: center;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 0.5rem;
	}

	.symbol {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.price {
		font-size: 1.5rem;
		font-weight: 600;
		color: #e5e7eb;
		text-align: center;
	}

	.change {
		font-size: 1.5rem;
		font-weight: 700;
		text-align: right;
	}

	.change.positive {
		color: #10b981;
	}

	.change.negative {
		color: #ef4444;
	}

	.countdown-section {
		text-align: center;
		padding: 3rem;
		background: rgba(255, 107, 53, 0.1);
		border: 2px solid rgba(255, 107, 53, 0.3);
		border-radius: 1rem;
		margin-top: 8rem;
	}

	.countdown-label {
		font-size: 2rem;
		color: #9ca3af;
		margin-bottom: 2rem;
	}

	.countdown-display {
		font-size: 6rem;
		font-weight: 700;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		font-family: monospace;
	}

	.signals-list {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.signal-item {
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
	}

	.signal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.signal-symbol {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.signal-badge {
		padding: 0.5rem 1rem;
		border-radius: 9999px;
		font-size: 1rem;
		font-weight: 700;
	}

	.signal-badge.long {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.signal-badge.short {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.signal-score {
		font-size: 1.5rem;
		font-weight: 700;
		color: #fbbf24;
		margin-bottom: 0.5rem;
	}

	.signal-entry {
		font-size: 1.25rem;
		color: #e5e7eb;
	}

	.overlay-footer {
		padding: 2rem 3rem;
		background: rgba(255, 255, 255, 0.05);
		border-top: 2px solid rgba(255, 107, 53, 0.3);
	}

	.donation-section {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 2rem;
	}

	.donation-label {
		font-size: 1.5rem;
		font-weight: 600;
		color: #e5e7eb;
	}

	.donation-buttons {
		display: flex;
		gap: 1rem;
	}

	.donate-btn {
		padding: 1rem 2rem;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		border: none;
		border-radius: 0.5rem;
		color: #fff;
		font-size: 1.25rem;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.3s;
	}

	.donate-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(255, 107, 53, 0.4);
	}
</style>
