<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { marketDataService, type MarketSummary } from '$lib/services/marketData';
	import type { Position } from '$lib/brokers/types';

	let marketData = $state<MarketSummary | null>(null);
	let positions: Position[] = $state([]);
	let portfolioStats = $state({
		totalValue: 125430.50,
		dayPnL: 2845.30,
		dayPnLPercent: 2.32,
		winRate: 68.5,
		activePositions: 3
	});

	onMount(async () => {
		// Fetch initial data
		marketData = await marketDataService.fetchMarketSummary();
		
		// Start auto-updates
		marketDataService.startAutoUpdate((data) => {
			marketData = data;
		}, 10000);

		// Mock positions for demo
		positions = [
			{ symbol: 'NVDA', quantity: 100, avgPrice: 850.00, currentPrice: 875.50, unrealizedPnL: 2550, unrealizedPnLPercent: 3.0, side: 'long' },
			{ symbol: 'TSLA', quantity: 50, avgPrice: 240.00, currentPrice: 245.80, unrealizedPnL: 290, unrealizedPnLPercent: 2.42, side: 'long' },
			{ symbol: 'SPY', quantity: 20, avgPrice: 445.00, currentPrice: 450.25, unrealizedPnL: 105, unrealizedPnLPercent: 1.18, side: 'long' }
		];
	});

	onDestroy(() => {
		marketDataService.stopAutoUpdate();
	});
</script>

<div class="dashboard-page">
	<header class="page-header">
		<div>
			<h1>📊 Dashboard Principal</h1>
			<p class="subtitle">Aperçu de votre portefeuille et du marché</p>
		</div>
		<div class="header-actions">
			<button class="btn-secondary">⚙️ Settings</button>
			<button class="btn-primary">🔔 Alerts</button>
		</div>
	</header>

	<!-- Portfolio Overview Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-header">
				<span class="stat-icon">💰</span>
				<span class="stat-label">Portfolio Value</span>
			</div>
			<div class="stat-value">${portfolioStats.totalValue.toLocaleString()}</div>
			<div class="stat-change positive">
				+{portfolioStats.dayPnLPercent}% Today
			</div>
		</div>

		<div class="stat-card">
			<div class="stat-header">
				<span class="stat-icon">📈</span>
				<span class="stat-label">Day P&L</span>
			</div>
			<div class="stat-value" class:positive={portfolioStats.dayPnL > 0}>
				${portfolioStats.dayPnL.toLocaleString()}
			</div>
			<div class="stat-change positive">
				+{portfolioStats.dayPnLPercent}%
			</div>
		</div>

		<div class="stat-card">
			<div class="stat-header">
				<span class="stat-icon">🎯</span>
				<span class="stat-label">Win Rate</span>
			</div>
			<div class="stat-value">{portfolioStats.winRate}%</div>
			<div class="stat-change">Last 30 days</div>
		</div>

		<div class="stat-card">
			<div class="stat-header">
				<span class="stat-icon">📊</span>
				<span class="stat-label">Active Positions</span>
			</div>
			<div class="stat-value">{portfolioStats.activePositions}</div>
			<div class="stat-change">Currently open</div>
		</div>
	</div>

	<!-- Market Overview -->
	{#if marketData}
		<div class="section">
			<h2 class="section-title">Market Overview</h2>
			<div class="market-grid">
				{#each marketData.indices as index}
					<div class="market-card">
						<div class="market-symbol">{index.symbol}</div>
						<div class="market-name">{index.name}</div>
						<div class="market-price">${index.price.toFixed(2)}</div>
						<div class="market-change" class:positive={index.changePercent > 0} class:negative={index.changePercent < 0}>
							{index.changePercent > 0 ? '+' : ''}{index.changePercent.toFixed(2)}%
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Active Positions -->
	<div class="section">
		<h2 class="section-title">Active Positions</h2>
		<div class="positions-table">
			<div class="table-header">
				<div class="table-cell">Symbol</div>
				<div class="table-cell">Quantity</div>
				<div class="table-cell">Avg Price</div>
				<div class="table-cell">Current Price</div>
				<div class="table-cell">P&L</div>
				<div class="table-cell">%</div>
			</div>
			{#each positions as position}
				<div class="table-row">
					<div class="table-cell">
						<span class="symbol">{position.symbol}</span>
					</div>
					<div class="table-cell">{position.quantity}</div>
					<div class="table-cell">${position.avgPrice.toFixed(2)}</div>
					<div class="table-cell">${position.currentPrice.toFixed(2)}</div>
					<div class="table-cell" class:positive={position.unrealizedPnL > 0} class:negative={position.unrealizedPnL < 0}>
						${position.unrealizedPnL.toFixed(2)}
					</div>
					<div class="table-cell" class:positive={position.unrealizedPnLPercent > 0} class:negative={position.unrealizedPnLPercent < 0}>
						{position.unrealizedPnLPercent > 0 ? '+' : ''}{position.unrealizedPnLPercent.toFixed(2)}%
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- Recent Signals -->
	<div class="section">
		<h2 class="section-title">Recent Signals</h2>
		<div class="signals-container">
			<div class="signal-card">
				<div class="signal-header">
					<span class="signal-symbol">NVDA</span>
					<span class="signal-badge long">LONG</span>
				</div>
				<div class="signal-score">Score: <span class="score-value">85</span></div>
				<div class="signal-details">
					<div>Entry: $870.00</div>
					<div>Target: $920.00</div>
					<div>R:R: 3:1</div>
				</div>
				<div class="signal-time">5 minutes ago</div>
			</div>

			<div class="signal-card">
				<div class="signal-header">
					<span class="signal-symbol">TSLA</span>
					<span class="signal-badge long">LONG</span>
				</div>
				<div class="signal-score">Score: <span class="score-value">72</span></div>
				<div class="signal-details">
					<div>Entry: $245.00</div>
					<div>Target: $265.00</div>
					<div>R:R: 2.5:1</div>
				</div>
				<div class="signal-time">12 minutes ago</div>
			</div>

			<div class="signal-card">
				<div class="signal-header">
					<span class="signal-symbol">AAPL</span>
					<span class="signal-badge short">SHORT</span>
				</div>
				<div class="signal-score">Score: <span class="score-value">68</span></div>
				<div class="signal-details">
					<div>Entry: $178.50</div>
					<div>Target: $170.00</div>
					<div>R:R: 2:1</div>
				</div>
				<div class="signal-time">28 minutes ago</div>
			</div>
		</div>
	</div>
</div>

<style>
	.dashboard-page {
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

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-primary, .btn-secondary {
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		color: #fff;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(255, 107, 53, 0.4);
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.05);
		color: #e5e7eb;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.btn-secondary:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s;
	}

	.stat-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 107, 53, 0.3);
		transform: translateY(-4px);
	}

	.stat-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.stat-icon {
		font-size: 1.5rem;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 0.5rem;
	}

	.stat-value.positive {
		color: #10b981;
	}

	.stat-change {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.stat-change.positive {
		color: #10b981;
	}

	.section {
		margin-bottom: 2rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 1rem;
	}

	.market-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 1rem;
	}

	.market-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1rem;
	}

	.market-symbol {
		font-size: 1.125rem;
		font-weight: 700;
		color: #ff6b35;
		margin-bottom: 0.25rem;
	}

	.market-name {
		font-size: 0.75rem;
		color: #6b7280;
		margin-bottom: 0.75rem;
	}

	.market-price {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f9fafb;
		margin-bottom: 0.25rem;
	}

	.market-change {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.market-change.positive {
		color: #10b981;
	}

	.market-change.negative {
		color: #ef4444;
	}

	.positions-table {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.table-header {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr;
		background: rgba(255, 107, 53, 0.1);
		padding: 1rem;
		font-weight: 600;
		color: #9ca3af;
		font-size: 0.875rem;
	}

	.table-row {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr;
		padding: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.table-row:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.table-cell {
		color: #e5e7eb;
	}

	.table-cell.positive {
		color: #10b981;
	}

	.table-cell.negative {
		color: #ef4444;
	}

	.symbol {
		font-weight: 700;
		color: #ff6b35;
	}

	.signals-container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.signal-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.25rem;
	}

	.signal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.signal-symbol {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.signal-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
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
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.75rem;
	}

	.score-value {
		font-size: 1.125rem;
		font-weight: 700;
		color: #fbbf24;
	}

	.signal-details {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		margin-bottom: 0.75rem;
		font-size: 0.875rem;
		color: #e5e7eb;
	}

	.signal-time {
		font-size: 0.75rem;
		color: #6b7280;
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}

		.table-header, .table-row {
			grid-template-columns: repeat(3, 1fr);
		}

		.table-cell:nth-child(n+4) {
			display: none;
		}

		.signals-container {
			grid-template-columns: 1fr;
		}
	}
</style>
