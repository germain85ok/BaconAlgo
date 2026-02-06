<script lang="ts">
	import { riskEngine, type VaRResult, type StressTestScenario } from '$lib/risk/riskEngine';
	import type { Position } from '$lib/brokers/types';

	let portfolioValue = $state(125430.50);
	let var95 = $state<VaRResult | null>(null);
	let var99 = $state<VaRResult | null>(null);
	let stressTests = $state<StressTestScenario[]>([]);
	let killSwitchActive = $state(false);

	// Mock positions
	let positions: Position[] = [
		{ symbol: 'NVDA', quantity: 100, avgPrice: 850.00, currentPrice: 875.50, unrealizedPnL: 2550, unrealizedPnLPercent: 3.0, side: 'long' },
		{ symbol: 'TSLA', quantity: 50, avgPrice: 240.00, currentPrice: 245.80, unrealizedPnL: 290, unrealizedPnLPercent: 2.42, side: 'long' },
		{ symbol: 'SPY', quantity: 20, avgPrice: 445.00, currentPrice: 450.25, unrealizedPnL: 105, unrealizedPnLPercent: 1.18, side: 'long' }
	];

	$effect(() => {
		var95 = riskEngine.calculateVaR(positions, 0.95);
		var99 = riskEngine.calculateVaR(positions, 0.99);
		stressTests = riskEngine.runStressTests(positions);
	});

	async function activateKillSwitch() {
		if (confirm('⚠️ This will close ALL positions and cancel ALL orders. Are you sure?')) {
			killSwitchActive = true;
			// In production, call the broker's kill switch
			setTimeout(() => {
				killSwitchActive = false;
				alert('✅ Kill switch executed. All positions closed.');
			}, 2000);
		}
	}
</script>

<div class="risk-page">
	<header class="page-header">
		<div>
			<h1>⚠️ Risk Management</h1>
			<p class="subtitle">Monitor and control your portfolio risk</p>
		</div>
		<button class="kill-switch" onclick={activateKillSwitch} class:active={killSwitchActive}>
			{killSwitchActive ? '🔴 CLOSING...' : '🔴 KILL SWITCH'}
		</button>
	</header>

	<!-- Value at Risk -->
	<div class="section">
		<h2 class="section-title">📊 Value at Risk (VaR)</h2>
		<div class="var-grid">
			<div class="var-card">
				<div class="var-header">
					<span class="var-label">VaR 95%</span>
					<span class="confidence-badge">95% Confidence</span>
				</div>
				<div class="var-value">${var95?.var95.toLocaleString() || '0'}</div>
				<div class="var-description">
					Maximum expected loss over 1 day with 95% confidence
				</div>
				<div class="var-percent">
					{var95 ? ((var95.var95 / var95.portfolioValue) * 100).toFixed(2) : '0'}% of portfolio
				</div>
			</div>

			<div class="var-card">
				<div class="var-header">
					<span class="var-label">VaR 99%</span>
					<span class="confidence-badge extreme">99% Confidence</span>
				</div>
				<div class="var-value">${var99?.var99.toLocaleString() || '0'}</div>
				<div class="var-description">
					Maximum expected loss over 1 day with 99% confidence
				</div>
				<div class="var-percent">
					{var99 ? ((var99.var99 / var99.portfolioValue) * 100).toFixed(2) : '0'}% of portfolio
				</div>
			</div>
		</div>
	</div>

	<!-- Stress Testing -->
	<div class="section">
		<h2 class="section-title">🔬 Stress Testing</h2>
		<p class="section-description">Estimated portfolio impact under various market scenarios</p>
		<div class="stress-grid">
			{#each stressTests as scenario}
				<div class="stress-card">
					<div class="stress-header">
						<span class="stress-icon">
							{#if scenario.marketDrop <= 0.10}
								⚠️
							{:else if scenario.marketDrop <= 0.20}
								🔴
							{:else}
								💀
							{/if}
						</span>
						<span class="stress-name">{scenario.name}</span>
					</div>
					<div class="stress-description">{scenario.description}</div>
					<div class="stress-loss">-${scenario.estimatedLoss.toLocaleString()}</div>
					<div class="stress-percent">
						-{(scenario.marketDrop * 100).toFixed(0)}% market drop
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- Exposure Limits -->
	<div class="section">
		<h2 class="section-title">📏 Exposure Limits</h2>
		<div class="limits-grid">
			<div class="limit-card">
				<div class="limit-header">
					<span>Active Positions</span>
					<span class="limit-status ok">OK</span>
				</div>
				<div class="limit-bar">
					<div class="limit-fill" style="width: 60%"></div>
				</div>
				<div class="limit-values">
					<span>3 / 5 positions</span>
					<span>60%</span>
				</div>
			</div>

			<div class="limit-card">
				<div class="limit-header">
					<span>Max Drawdown</span>
					<span class="limit-status ok">OK</span>
				</div>
				<div class="limit-bar">
					<div class="limit-fill" style="width: 35%"></div>
				</div>
				<div class="limit-values">
					<span>3.5% / 10%</span>
					<span>35%</span>
				</div>
			</div>

			<div class="limit-card">
				<div class="limit-header">
					<span>Daily Loss Limit</span>
					<span class="limit-status ok">OK</span>
				</div>
				<div class="limit-bar">
					<div class="limit-fill positive" style="width: 0%"></div>
				</div>
				<div class="limit-values">
					<span>+$2,845 / -$5,000</span>
					<span>0%</span>
				</div>
			</div>
		</div>
	</div>

	<!-- Drawdown Monitoring -->
	<div class="section">
		<h2 class="section-title">📉 Drawdown Monitoring</h2>
		<div class="drawdown-grid">
			<div class="drawdown-card">
				<div class="dd-label">Current Drawdown</div>
				<div class="dd-value">-3.5%</div>
				<div class="dd-bar">
					<div class="dd-fill" style="width: 35%"></div>
				</div>
			</div>

			<div class="drawdown-card">
				<div class="dd-label">Max Drawdown (All-Time)</div>
				<div class="dd-value">-12.8%</div>
				<div class="dd-date">Nov 15, 2025</div>
			</div>

			<div class="drawdown-card">
				<div class="dd-label">Recovery to High Water Mark</div>
				<div class="dd-value">+3.6%</div>
				<div class="dd-description">$4,480 needed</div>
			</div>
		</div>
	</div>
</div>

<style>
	.risk-page {
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

	.kill-switch {
		padding: 1rem 2rem;
		background: linear-gradient(135deg, #dc2626 0%, #991b1b 100%);
		border: 2px solid #ef4444;
		border-radius: 0.5rem;
		color: #fff;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
		animation: pulse 2s ease-in-out infinite;
	}

	.kill-switch:hover {
		transform: scale(1.05);
		box-shadow: 0 0 30px rgba(239, 68, 68, 0.6);
	}

	.kill-switch.active {
		background: #991b1b;
		animation: none;
	}

	@keyframes pulse {
		0%, 100% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.7); }
		50% { box-shadow: 0 0 20px 10px rgba(239, 68, 68, 0); }
	}

	.section {
		margin-bottom: 3rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
		margin-bottom: 0.5rem;
	}

	.section-description {
		color: #9ca3af;
		margin-bottom: 1.5rem;
	}

	.var-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
		gap: 1.5rem;
	}

	.var-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
	}

	.var-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.var-label {
		font-size: 1.125rem;
		font-weight: 600;
		color: #e5e7eb;
	}

	.confidence-badge {
		padding: 0.25rem 0.75rem;
		background: rgba(59, 130, 246, 0.2);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
		color: #60a5fa;
	}

	.confidence-badge.extreme {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.var-value {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fbbf24;
		margin-bottom: 0.75rem;
	}

	.var-description {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.5rem;
	}

	.var-percent {
		font-size: 1rem;
		font-weight: 600;
		color: #6b7280;
	}

	.stress-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.stress-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.25rem;
	}

	.stress-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.stress-icon {
		font-size: 1.5rem;
	}

	.stress-name {
		font-size: 1rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.stress-description {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.75rem;
	}

	.stress-loss {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ef4444;
		margin-bottom: 0.25rem;
	}

	.stress-percent {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.limits-grid {
		display: grid;
		gap: 1rem;
	}

	.limit-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.25rem;
	}

	.limit-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
		font-weight: 600;
		color: #e5e7eb;
	}

	.limit-status {
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.limit-status.ok {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.limit-bar {
		height: 0.5rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.limit-fill {
		height: 100%;
		background: linear-gradient(90deg, #fbbf24 0%, #f59e0b 100%);
		border-radius: 9999px;
		transition: width 0.5s ease;
	}

	.limit-fill.positive {
		background: linear-gradient(90deg, #10b981 0%, #059669 100%);
	}

	.limit-values {
		display: flex;
		justify-content: space-between;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.drawdown-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.drawdown-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
	}

	.dd-label {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.75rem;
	}

	.dd-value {
		font-size: 2rem;
		font-weight: 700;
		color: #ef4444;
		margin-bottom: 0.75rem;
	}

	.dd-bar {
		height: 0.5rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		overflow: hidden;
	}

	.dd-fill {
		height: 100%;
		background: linear-gradient(90deg, #ef4444 0%, #dc2626 100%);
		border-radius: 9999px;
	}

	.dd-date, .dd-description {
		font-size: 0.875rem;
		color: #6b7280;
	}

	@media (max-width: 768px) {
		.var-grid, .stress-grid, .drawdown-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
