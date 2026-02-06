<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { apiClient } from '$lib/services/apiClient';
	import type { Signal, Portfolio } from '$lib/types';
	
	let portfolio: Portfolio | null = null;
	let activeSignals: Signal[] = [];
	let dailyPnL = 0;
	let dailyPnLPercent = 0;
	let unsubscribe: (() => void)[] = [];

	let showPnL = true;
	let showSignals = true;
	let compactMode = true;

	onMount(() => {
		loadData();

		const unsub1 = apiClient.subscribe('POSITION_UPDATE', (data: Portfolio) => {
			portfolio = data;
			dailyPnL = data.daily_pnl;
			dailyPnLPercent = (data.daily_pnl / data.total_value) * 100;
		});

		const unsub2 = apiClient.subscribe('SIGNAL', (data: Signal) => {
			if (data.status === 'ACTIVE') {
				activeSignals = [data, ...activeSignals.slice(0, 4)];
			}
		});

		unsubscribe = [unsub1, unsub2];
	});

	onDestroy(() => {
		unsubscribe.forEach(unsub => unsub());
	});

	async function loadData() {
		const portfolioResponse = await apiClient.getPortfolio();
		if (portfolioResponse.success && portfolioResponse.data) {
			portfolio = portfolioResponse.data;
			dailyPnL = portfolio.daily_pnl;
			dailyPnLPercent = (portfolio.daily_pnl / portfolio.total_value) * 100;
		}

		const signalsResponse = await apiClient.getSignals(5);
		if (signalsResponse.success && signalsResponse.data) {
			activeSignals = signalsResponse.data.filter(s => s.status === 'ACTIVE');
		}
	}
</script>

<svelte:head>
	<title>BaconAlgo Stream Overlay</title>
</svelte:head>

<div class="overlay-container">
	{#if showPnL && portfolio}
		<div class="pnl-widget glass-overlay {dailyPnL >= 0 ? 'positive' : 'negative'}">
			<div class="pnl-label">P&L Quotidien</div>
			<div class="pnl-amount">
				{dailyPnL >= 0 ? '+' : ''}{dailyPnL.toLocaleString('fr-CA', { 
					style: 'currency', 
					currency: 'USD' 
				})}
			</div>
			<div class="pnl-percent">
				{dailyPnL >= 0 ? '↑' : '↓'} {Math.abs(dailyPnLPercent).toFixed(2)}%
			</div>
		</div>
	{/if}

	{#if showSignals && activeSignals.length > 0}
		<div class="signals-widget glass-overlay">
			<div class="widget-header">
				<span class="pulse-dot"></span>
				<span class="widget-title">Signaux Actifs</span>
			</div>
			
			<div class="signals-list">
				{#each activeSignals.slice(0, 3) as signal}
					<div class="signal-item {signal.direction.toLowerCase()}">
						<div class="signal-main">
							<span class="signal-symbol">{signal.symbol}</span>
							<span class="signal-direction {signal.direction.toLowerCase()}">
								{signal.direction === 'LONG' ? '📈' : '📉'} {signal.direction}
							</span>
						</div>
						{#if !compactMode}
							<div class="signal-details">
								<div class="detail-item">
									<span class="label">Entrée:</span>
									<span class="value">${signal.entry_price.toFixed(2)}</span>
								</div>
								<div class="detail-item">
									<span class="label">TP:</span>
									<span class="value text-green">${signal.take_profit.toFixed(2)}</span>
								</div>
							</div>
						{/if}
						<div class="signal-footer">
							<span class="confidence">
								{signal.confidence.toFixed(0)}% confiance
							</span>
							<span class="timeframe">{signal.timeframe}</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	:global(body) {
		background: transparent !important;
		overflow: hidden;
	}

	.overlay-container {
		position: fixed;
		top: 20px;
		right: 20px;
		display: flex;
		flex-direction: column;
		gap: 15px;
		max-width: 350px;
		font-family: 'Inter', sans-serif;
	}

	.glass-overlay {
		background: rgba(10, 10, 15, 0.85);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 107, 53, 0.3);
		border-radius: 12px;
		padding: 16px;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4),
					0 0 20px rgba(255, 107, 53, 0.2);
	}

	.pnl-widget {
		text-align: center;
	}

	.pnl-label {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.6);
		text-transform: uppercase;
		letter-spacing: 1px;
		margin-bottom: 8px;
	}

	.pnl-amount {
		font-size: 32px;
		font-weight: 800;
		font-family: 'Orbitron', sans-serif;
		margin-bottom: 4px;
	}

	.pnl-widget.positive .pnl-amount {
		color: #10b981;
		text-shadow: 0 0 20px rgba(16, 185, 129, 0.5);
	}

	.pnl-widget.negative .pnl-amount {
		color: #ef4444;
		text-shadow: 0 0 20px rgba(239, 68, 68, 0.5);
	}

	.pnl-percent {
		font-size: 16px;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.8);
	}

	.widget-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 12px;
	}

	.pulse-dot {
		width: 8px;
		height: 8px;
		background: #ff6b35;
		border-radius: 50%;
		animation: pulse 2s ease-in-out infinite;
	}

	.widget-title {
		font-size: 14px;
		font-weight: 700;
		color: #ff6b35;
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.signals-list {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.signal-item {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
		padding: 12px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.signal-item.long {
		border-left: 3px solid #10b981;
	}

	.signal-item.short {
		border-left: 3px solid #ef4444;
	}

	.signal-main {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 8px;
	}

	.signal-symbol {
		font-size: 16px;
		font-weight: 700;
		font-family: 'Orbitron', sans-serif;
		color: #ff6b35;
	}

	.signal-direction {
		font-size: 14px;
		font-weight: 700;
		padding: 4px 8px;
		border-radius: 4px;
	}

	.signal-direction.long {
		color: #10b981;
		background: rgba(16, 185, 129, 0.1);
	}

	.signal-direction.short {
		color: #ef4444;
		background: rgba(239, 68, 68, 0.1);
	}

	.signal-details {
		display: flex;
		gap: 12px;
		margin-bottom: 8px;
		font-size: 12px;
	}

	.detail-item {
		display: flex;
		gap: 4px;
	}

	.detail-item .label {
		color: rgba(255, 255, 255, 0.5);
	}

	.detail-item .value {
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
		color: white;
	}

	.detail-item .text-green {
		color: #10b981;
	}

	.signal-footer {
		display: flex;
		justify-content: space-between;
		font-size: 11px;
		color: rgba(255, 255, 255, 0.6);
	}

	.confidence {
		font-weight: 600;
		color: #ff6b35;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
			box-shadow: 0 0 0 0 rgba(255, 107, 53, 0.7);
		}
		50% {
			opacity: 0.8;
			box-shadow: 0 0 0 8px rgba(255, 107, 53, 0);
		}
	}
</style>
