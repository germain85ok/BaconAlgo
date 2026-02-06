<script lang="ts">
	import { orderFlowAnalyzer, type OrderFlowData, type OptionsFlowData } from '$lib/institutional/orderFlow';

	let selectedSymbol = $state('SPY');
	let orderFlow = $state<OrderFlowData | null>(null);
	let optionsFlow = $state<OptionsFlowData | null>(null);
	let smartMoneyIndex = $state(0);

	$effect(() => {
		// Simulate order flow data
		orderFlow = orderFlowAnalyzer.analyzeDelta(
			Math.random() * 1000000,
			Math.random() * 1000000
		);
		orderFlow.symbol = selectedSymbol;

		optionsFlow = orderFlowAnalyzer.analyzeOptionsFlow(selectedSymbol);
		
		const darkPool = orderFlowAnalyzer.detectDarkPool(selectedSymbol);
		smartMoneyIndex = orderFlowAnalyzer.calculateSmartMoneyIndex(orderFlow, darkPool, optionsFlow);
	});
</script>

<div class="orderflow-page">
	<header class="page-header">
		<div>
			<h1>📊 Order Flow Analysis</h1>
			<p class="subtitle">Institutional tracking and smart money indicators</p>
		</div>
		<input type="text" bind:value={selectedSymbol} placeholder="Symbol" class="symbol-input" />
	</header>

	<!-- Smart Money Index -->
	<div class="section">
		<h2 class="section-title">💡 Smart Money Index</h2>
		<div class="smi-card">
			<div class="smi-value">{smartMoneyIndex}</div>
			<div class="smi-bar">
				<div class="smi-fill" style="width: {smartMoneyIndex}%"></div>
			</div>
			<div class="smi-labels">
				<span>Bearish</span>
				<span>Neutral</span>
				<span>Bullish</span>
			</div>
		</div>
	</div>

	<!-- Volume Delta -->
	{#if orderFlow}
		<div class="section">
			<h2 class="section-title">📈 Volume Delta</h2>
			<div class="delta-grid">
				<div class="delta-card">
					<div class="delta-label">Buy Volume</div>
					<div class="delta-value buy">{orderFlow.buyVolume.toLocaleString()}</div>
				</div>
				<div class="delta-card">
					<div class="delta-label">Sell Volume</div>
					<div class="delta-value sell">{orderFlow.sellVolume.toLocaleString()}</div>
				</div>
				<div class="delta-card">
					<div class="delta-label">Delta</div>
					<div class="delta-value" class:buy={orderFlow.delta > 0} class:sell={orderFlow.delta < 0}>
						{orderFlow.delta > 0 ? '+' : ''}{orderFlow.delta.toLocaleString()}
					</div>
				</div>
				<div class="delta-card">
					<div class="delta-label">Cumulative Delta</div>
					<div class="delta-value" class:buy={orderFlow.cumulativeDelta > 0} class:sell={orderFlow.cumulativeDelta < 0}>
						{orderFlow.cumulativeDelta > 0 ? '+' : ''}{orderFlow.cumulativeDelta.toLocaleString()}
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Options Flow -->
	{#if optionsFlow}
		<div class="section">
			<h2 class="section-title">📊 Options Flow</h2>
			<div class="options-grid">
				<div class="options-card">
					<div class="options-header">
						<span class="options-label">Put/Call Ratio</span>
						{#if optionsFlow.unusualActivity}
							<span class="unusual-badge">🚨 Unusual Activity</span>
						{/if}
					</div>
					<div class="options-value">{optionsFlow.putCallRatio.toFixed(2)}</div>
					<div class="options-interpretation">
						{#if optionsFlow.putCallRatio > 1.2}
							Bearish (More Puts)
						{:else if optionsFlow.putCallRatio < 0.8}
							Bullish (More Calls)
						{:else}
							Neutral
						{/if}
					</div>
				</div>

				<div class="options-card">
					<div class="options-label">Call Volume</div>
					<div class="options-value call">{optionsFlow.callVolume.toLocaleString()}</div>
				</div>

				<div class="options-card">
					<div class="options-label">Put Volume</div>
					<div class="options-value put">{optionsFlow.putVolume.toLocaleString()}</div>
				</div>

				<div class="options-card">
					<div class="options-label">Implied Volatility</div>
					<div class="options-value">{(optionsFlow.impliedVolatility * 100).toFixed(1)}%</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Dark Pool Activity -->
	<div class="section">
		<h2 class="section-title">🕶️ Dark Pool Data</h2>
		<div class="darkpool-card">
			<div class="dp-info">
				<div class="dp-item">
					<span class="dp-label">Volume:</span>
					<span class="dp-value">1.2M shares</span>
				</div>
				<div class="dp-item">
					<span class="dp-label">Avg Price:</span>
					<span class="dp-value">$450.25</span>
				</div>
				<div class="dp-item">
					<span class="dp-label">% of Total:</span>
					<span class="dp-value">38%</span>
				</div>
			</div>
			<div class="dp-chart">
				<div class="dp-bar" style="height: 60%"></div>
				<div class="dp-bar" style="height: 75%"></div>
				<div class="dp-bar" style="height: 45%"></div>
				<div class="dp-bar" style="height: 80%"></div>
				<div class="dp-bar" style="height: 65%"></div>
			</div>
		</div>
	</div>

	<!-- Institutional Tracking -->
	<div class="section">
		<h2 class="section-title">🏦 Institutional Activity</h2>
		<div class="institutional-grid">
			<div class="inst-card accumulation">
				<div class="inst-icon">📈</div>
				<div class="inst-type">Accumulation</div>
				<div class="inst-confidence">Confidence: 78%</div>
				<div class="inst-description">Institutions are buying</div>
			</div>

			<div class="inst-card neutral">
				<div class="inst-icon">➡️</div>
				<div class="inst-type">Neutral Flow</div>
				<div class="inst-confidence">No Clear Signal</div>
				<div class="inst-description">Balanced activity</div>
			</div>
		</div>
	</div>
</div>

<style>
	.orderflow-page {
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

	.symbol-input {
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #f9fafb;
		font-size: 1rem;
		font-weight: 600;
		width: 150px;
	}

	.section {
		margin-bottom: 3rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 1.5rem;
	}

	.smi-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 2rem;
	}

	.smi-value {
		font-size: 4rem;
		font-weight: 700;
		color: #fbbf24;
		text-align: center;
		margin-bottom: 1.5rem;
	}

	.smi-bar {
		height: 2rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		overflow: hidden;
		margin-bottom: 0.75rem;
	}

	.smi-fill {
		height: 100%;
		background: linear-gradient(90deg, #ef4444 0%, #fbbf24 50%, #10b981 100%);
		border-radius: 9999px;
		transition: width 0.5s ease;
	}

	.smi-labels {
		display: flex;
		justify-content: space-between;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.delta-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.delta-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
	}

	.delta-label {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.75rem;
	}

	.delta-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.delta-value.buy {
		color: #10b981;
	}

	.delta-value.sell {
		color: #ef4444;
	}

	.options-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.options-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
	}

	.options-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.options-label {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.unusual-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(239, 68, 68, 0.2);
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 700;
		color: #ef4444;
	}

	.options-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 0.5rem;
	}

	.options-value.call {
		color: #10b981;
	}

	.options-value.put {
		color: #ef4444;
	}

	.options-interpretation {
		font-size: 0.875rem;
		color: #fbbf24;
		font-weight: 600;
	}

	.darkpool-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 2rem;
	}

	.dp-info {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.dp-item {
		display: flex;
		justify-content: space-between;
	}

	.dp-label {
		color: #9ca3af;
	}

	.dp-value {
		color: #f9fafb;
		font-weight: 600;
	}

	.dp-chart {
		display: flex;
		align-items: flex-end;
		gap: 0.5rem;
		height: 150px;
	}

	.dp-bar {
		flex: 1;
		background: linear-gradient(180deg, #6366f1 0%, #3730a3 100%);
		border-radius: 0.25rem;
		min-height: 20px;
	}

	.institutional-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.inst-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
		text-align: center;
	}

	.inst-card.accumulation {
		border-left: 4px solid #10b981;
	}

	.inst-card.neutral {
		border-left: 4px solid #9ca3af;
	}

	.inst-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.inst-type {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 0.5rem;
	}

	.inst-confidence {
		font-size: 0.875rem;
		color: #fbbf24;
		margin-bottom: 0.5rem;
	}

	.inst-description {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	@media (max-width: 768px) {
		.delta-grid, .options-grid, .institutional-grid {
			grid-template-columns: 1fr;
		}

		.darkpool-card {
			grid-template-columns: 1fr;
		}
	}
</style>
