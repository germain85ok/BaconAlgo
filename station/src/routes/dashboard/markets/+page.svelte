<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { marketDataService, type MarketSummary } from '$lib/services/marketData';

	let marketData = $state<MarketSummary | null>(null);
	let selectedTab = $state<'indices' | 'crypto' | 'commodities' | 'movers'>('indices');

	onMount(async () => {
		marketData = await marketDataService.fetchMarketSummary();
		
		marketDataService.startAutoUpdate((data) => {
			marketData = data;
		}, 10000);
	});

	onDestroy(() => {
		marketDataService.stopAutoUpdate();
	});
</script>

<div class="markets-page">
	<header class="page-header">
		<div>
			<h1>🌍 Markets Overview</h1>
			<p class="subtitle">Real-time market data across indices, crypto, and commodities</p>
		</div>
		<div class="last-update">
			{#if marketData}
				Last updated: {marketData.lastUpdated.toLocaleTimeString()}
			{/if}
		</div>
	</header>

	<!-- Tabs -->
	<div class="tabs">
		<button class="tab" class:active={selectedTab === 'indices'} onclick={() => selectedTab = 'indices'}>
			📊 Indices
		</button>
		<button class="tab" class:active={selectedTab === 'crypto'} onclick={() => selectedTab = 'crypto'}>
			₿ Crypto (Top 20)
		</button>
		<button class="tab" class:active={selectedTab === 'commodities'} onclick={() => selectedTab = 'commodities'}>
			💎 Commodities
		</button>
		<button class="tab" class:active={selectedTab === 'movers'} onclick={() => selectedTab = 'movers'}>
			🚀 Top Movers
		</button>
	</div>

	{#if marketData}
		<!-- Indices -->
		{#if selectedTab === 'indices'}
			<div class="content-section">
				<h2 class="section-title">Major Indices</h2>
				<div class="markets-grid">
					{#each marketData.indices as index}
						<div class="market-card">
							<div class="card-header">
								<div class="symbol">{index.symbol}</div>
								<div class="change-badge" class:positive={index.changePercent > 0} class:negative={index.changePercent < 0}>
									{index.changePercent > 0 ? '▲' : '▼'} {Math.abs(index.changePercent).toFixed(2)}%
								</div>
							</div>
							<div class="name">{index.name}</div>
							<div class="price">${index.price.toFixed(2)}</div>
							<div class="change" class:positive={index.change > 0} class:negative={index.change < 0}>
								{index.change > 0 ? '+' : ''}{index.change.toFixed(2)}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Crypto -->
		{#if selectedTab === 'crypto'}
			<div class="content-section">
				<h2 class="section-title">Top 20 Cryptocurrencies</h2>
				<div class="crypto-table">
					<div class="table-header">
						<div>Symbol</div>
						<div>Price</div>
						<div>24h Change</div>
						<div>24h Volume</div>
						<div>Market Cap</div>
					</div>
					{#each marketData.crypto as crypto}
						<div class="table-row">
							<div class="crypto-symbol">{crypto.symbol}</div>
							<div class="price">${crypto.price.toLocaleString(undefined, {minimumFractionDigits: 2, maximumFractionDigits: 2})}</div>
							<div class="change" class:positive={crypto.changePercent24h > 0} class:negative={crypto.changePercent24h < 0}>
								{crypto.changePercent24h > 0 ? '+' : ''}{crypto.changePercent24h.toFixed(2)}%
							</div>
							<div>${(crypto.volume24h / 1e9).toFixed(2)}B</div>
							<div>${(crypto.marketCap / 1e9).toFixed(2)}B</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Commodities -->
		{#if selectedTab === 'commodities'}
			<div class="content-section">
				<h2 class="section-title">Commodities</h2>
				<div class="markets-grid">
					{#each marketData.commodities as commodity}
						<div class="market-card commodity">
							<div class="card-header">
								<div class="symbol">{commodity.symbol}</div>
								<div class="change-badge" class:positive={commodity.changePercent > 0} class:negative={commodity.changePercent < 0}>
									{commodity.changePercent > 0 ? '▲' : '▼'} {Math.abs(commodity.changePercent).toFixed(2)}%
								</div>
							</div>
							<div class="name">{commodity.name}</div>
							<div class="price">${commodity.price.toFixed(2)}</div>
							<div class="change" class:positive={commodity.change > 0} class:negative={commodity.change < 0}>
								{commodity.change > 0 ? '+' : ''}{commodity.change.toFixed(2)}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Top Movers -->
		{#if selectedTab === 'movers'}
			<div class="content-section">
				<div class="movers-container">
					<div class="movers-section">
						<h2 class="section-title gainers">🚀 Top Gainers</h2>
						<div class="movers-list">
							{#each marketData.gainers as gainer}
								<div class="mover-card gainers">
									<div class="mover-header">
										<span class="symbol">{gainer.symbol}</span>
										<span class="badge positive">+{gainer.changePercent.toFixed(2)}%</span>
									</div>
									<div class="name">{gainer.name}</div>
									<div class="details">
										<span class="price">${gainer.price.toFixed(2)}</span>
										<span class="volume">Vol: {(gainer.volume / 1e6).toFixed(1)}M</span>
									</div>
								</div>
							{/each}
						</div>
					</div>

					<div class="movers-section">
						<h2 class="section-title losers">📉 Top Losers</h2>
						<div class="movers-list">
							{#each marketData.losers as loser}
								<div class="mover-card losers">
									<div class="mover-header">
										<span class="symbol">{loser.symbol}</span>
										<span class="badge negative">{loser.changePercent.toFixed(2)}%</span>
									</div>
									<div class="name">{loser.name}</div>
									<div class="details">
										<span class="price">${loser.price.toFixed(2)}</span>
										<span class="volume">Vol: {(loser.volume / 1e6).toFixed(1)}M</span>
									</div>
								</div>
							{/each}
						</div>
					</div>
				</div>
			</div>
		{/if}
	{:else}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading market data...</p>
		</div>
	{/if}
</div>

<style>
	.markets-page {
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f9fafb;
		margin: 0;
	}

	.subtitle {
		color: #9ca3af;
		margin: 0.5rem 0 0 0;
	}

	.last-update {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		overflow-x: auto;
	}

	.tab {
		padding: 0.75rem 1.5rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #e5e7eb;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.tab.active {
		background: linear-gradient(135deg, rgba(255, 107, 53, 0.2), rgba(251, 191, 36, 0.2));
		border-color: rgba(255, 107, 53, 0.5);
		color: #fbbf24;
	}

	.content-section {
		margin-bottom: 2rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 1.5rem;
	}

	.section-title.gainers {
		color: #10b981;
	}

	.section-title.losers {
		color: #ef4444;
	}

	.markets-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.market-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s;
	}

	.market-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 107, 53, 0.3);
		transform: translateY(-4px);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.symbol {
		font-size: 1.25rem;
		font-weight: 700;
		color: #ff6b35;
	}

	.change-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.change-badge.positive {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.change-badge.negative {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.name {
		font-size: 0.875rem;
		color: #6b7280;
		margin-bottom: 0.75rem;
	}

	.price {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 0.375rem;
	}

	.change {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.change.positive {
		color: #10b981;
	}

	.change.negative {
		color: #ef4444;
	}

	.crypto-table {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.table-header {
		display: grid;
		grid-template-columns: 1fr 1.5fr 1fr 1.5fr 1.5fr;
		background: rgba(255, 107, 53, 0.1);
		padding: 1rem 1.5rem;
		font-weight: 600;
		color: #9ca3af;
		font-size: 0.875rem;
	}

	.table-row {
		display: grid;
		grid-template-columns: 1fr 1.5fr 1fr 1.5fr 1.5fr;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
		align-items: center;
	}

	.table-row:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.crypto-symbol {
		font-weight: 700;
		color: #ff6b35;
	}

	.movers-container {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 2rem;
	}

	.movers-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.mover-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.25rem;
		transition: all 0.2s;
	}

	.mover-card.gainers {
		border-left: 3px solid #10b981;
	}

	.mover-card.losers {
		border-left: 3px solid #ef4444;
	}

	.mover-card:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.mover-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.badge {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.badge.positive {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.badge.negative {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.details {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.75rem;
		font-size: 0.875rem;
	}

	.volume {
		color: #6b7280;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		color: #9ca3af;
	}

	.spinner {
		width: 50px;
		height: 50px;
		border: 4px solid rgba(255, 107, 53, 0.2);
		border-top: 4px solid #ff6b35;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}

	@media (max-width: 768px) {
		.markets-grid {
			grid-template-columns: 1fr;
		}

		.table-header, .table-row {
			grid-template-columns: 1fr 1fr 1fr;
		}

		.table-header div:nth-child(n+4),
		.table-row div:nth-child(n+4) {
			display: none;
		}

		.movers-container {
			grid-template-columns: 1fr;
		}
	}
</style>
