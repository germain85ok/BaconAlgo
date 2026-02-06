<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	
	// Types
	interface SystemMetrics {
		cpu_usage_pct: number;
		memory_usage_mb: number;
		memory_total_mb: number;
		uptime_secs: number;
		thread_count: number;
		network_rx_bps: number;
		network_tx_bps: number;
	}
	
	interface LatencyMetrics {
		avg_latency_ns: number;
		p50_latency_ns: number;
		p95_latency_ns: number;
		p99_latency_ns: number;
		min_latency_ns: number;
		max_latency_ns: number;
	}
	
	interface ThroughputMetrics {
		messages_per_sec: number;
		orders_per_sec: number;
		scans_per_minute: number;
		signals_per_minute: number;
	}
	
	interface TradingMetrics {
		active_positions: number;
		pending_orders: number;
		volume_24h: number;
		daily_pnl: number;
		total_pnl: number;
		win_rate: number;
	}
	
	interface PerformanceMetrics {
		system: SystemMetrics;
		latency: LatencyMetrics;
		throughput: ThroughputMetrics;
		trading: TradingMetrics;
		timestamp: number;
	}
	
	// State
	let metrics: PerformanceMetrics | null = null;
	let loading = true;
	let error = '';
	let refreshInterval: number;
	
	// API base URL
	const API_BASE = 'http://localhost:3000';
	
	// Fetch metrics from API
	async function fetchMetrics() {
		try {
			const response = await fetch(`${API_BASE}/api/metrics`);
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			metrics = await response.json();
			loading = false;
			error = '';
		} catch (e) {
			error = `Erreur: ${e instanceof Error ? e.message : 'Unknown error'}`;
			loading = false;
		}
	}
	
	// Format numbers
	function formatNumber(num: number, decimals: number = 0): string {
		return num.toLocaleString('fr-CA', { 
			minimumFractionDigits: decimals,
			maximumFractionDigits: decimals 
		});
	}
	
	// Format microseconds to readable format
	function formatLatency(ns: number): string {
		if (ns < 1000) {
			return `${ns}ns`;
		} else if (ns < 1_000_000) {
			return `${(ns / 1000).toFixed(1)}μs`;
		} else {
			return `${(ns / 1_000_000).toFixed(1)}ms`;
		}
	}
	
	// Format uptime
	function formatUptime(secs: number): string {
		const days = Math.floor(secs / 86400);
		const hours = Math.floor((secs % 86400) / 3600);
		const minutes = Math.floor((secs % 3600) / 60);
		
		if (days > 0) {
			return `${days}j ${hours}h`;
		} else if (hours > 0) {
			return `${hours}h ${minutes}m`;
		} else {
			return `${minutes}m`;
		}
	}
	
	// Lifecycle
	onMount(() => {
		fetchMetrics();
		refreshInterval = setInterval(fetchMetrics, 1000); // Refresh every second
	});
	
	onDestroy(() => {
		if (refreshInterval) {
			clearInterval(refreshInterval);
		}
	});
</script>

<div class="quantum-performance">
	<div class="header">
		<h2>🚀 Performance Quantique</h2>
		<div class="status-badge">
			{#if loading}
				<span class="status loading">⏳ Chargement...</span>
			{:else if error}
				<span class="status error">❌ {error}</span>
			{:else}
				<span class="status active">✅ Actif</span>
			{/if}
		</div>
	</div>
	
	{#if metrics}
		<!-- System Metrics -->
		<div class="metrics-grid">
			<div class="metric-card system">
				<h3>💻 Système</h3>
				<div class="metric-row">
					<span class="label">CPU:</span>
					<span class="value">{formatNumber(metrics.system.cpu_usage_pct, 1)}%</span>
				</div>
				<div class="metric-row">
					<span class="label">Mémoire:</span>
					<span class="value">
						{formatNumber(metrics.system.memory_usage_mb, 0)} MB / 
						{formatNumber(metrics.system.memory_total_mb, 0)} MB
					</span>
				</div>
				<div class="metric-row">
					<span class="label">Threads:</span>
					<span class="value">{metrics.system.thread_count}</span>
				</div>
				<div class="metric-row">
					<span class="label">Uptime:</span>
					<span class="value">{formatUptime(metrics.system.uptime_secs)}</span>
				</div>
			</div>
			
			<!-- Latency Metrics -->
			<div class="metric-card latency">
				<h3>⚡ Latence</h3>
				<div class="metric-row highlight">
					<span class="label">Moyenne:</span>
					<span class="value">{formatLatency(metrics.latency.avg_latency_ns)}</span>
				</div>
				<div class="metric-row">
					<span class="label">p50:</span>
					<span class="value">{formatLatency(metrics.latency.p50_latency_ns)}</span>
				</div>
				<div class="metric-row">
					<span class="label">p95:</span>
					<span class="value">{formatLatency(metrics.latency.p95_latency_ns)}</span>
				</div>
				<div class="metric-row">
					<span class="label">p99:</span>
					<span class="value">{formatLatency(metrics.latency.p99_latency_ns)}</span>
				</div>
				<div class="metric-row">
					<span class="label">Min:</span>
					<span class="value">{formatLatency(metrics.latency.min_latency_ns)}</span>
				</div>
				<div class="metric-row">
					<span class="label">Max:</span>
					<span class="value">{formatLatency(metrics.latency.max_latency_ns)}</span>
				</div>
			</div>
			
			<!-- Throughput Metrics -->
			<div class="metric-card throughput">
				<h3>🏎️ Throughput</h3>
				<div class="metric-row highlight">
					<span class="label">Messages/s:</span>
					<span class="value">{formatNumber(metrics.throughput.messages_per_sec)} msg/s</span>
				</div>
				<div class="metric-row">
					<span class="label">Ordres/s:</span>
					<span class="value">{formatNumber(metrics.throughput.orders_per_sec)}</span>
				</div>
				<div class="metric-row">
					<span class="label">Scans/min:</span>
					<span class="value">{formatNumber(metrics.throughput.scans_per_minute)}</span>
				</div>
				<div class="metric-row">
					<span class="label">Signaux/min:</span>
					<span class="value">{formatNumber(metrics.throughput.signals_per_minute)}</span>
				</div>
			</div>
			
			<!-- Trading Metrics -->
			<div class="metric-card trading">
				<h3>📈 Trading</h3>
				<div class="metric-row">
					<span class="label">Positions:</span>
					<span class="value">{metrics.trading.active_positions}</span>
				</div>
				<div class="metric-row">
					<span class="label">Ordres:</span>
					<span class="value">{metrics.trading.pending_orders}</span>
				</div>
				<div class="metric-row">
					<span class="label">Volume 24h:</span>
					<span class="value">${formatNumber(metrics.trading.volume_24h, 0)}</span>
				</div>
				<div class="metric-row highlight">
					<span class="label">P&L Jour:</span>
					<span class="value {metrics.trading.daily_pnl >= 0 ? 'positive' : 'negative'}">
						${formatNumber(metrics.trading.daily_pnl, 2)}
					</span>
				</div>
				<div class="metric-row">
					<span class="label">P&L Total:</span>
					<span class="value {metrics.trading.total_pnl >= 0 ? 'positive' : 'negative'}">
						${formatNumber(metrics.trading.total_pnl, 2)}
					</span>
				</div>
				<div class="metric-row">
					<span class="label">Win Rate:</span>
					<span class="value">{formatNumber(metrics.trading.win_rate, 1)}%</span>
				</div>
			</div>
		</div>
		
		<!-- Performance Target Indicators -->
		<div class="target-indicators">
			<div class="target-item">
				<span class="target-label">Cible Latence:</span>
				<span class="target-value">
					{#if metrics.latency.avg_latency_ns < 10000}
						<span class="achieved">✅ < 10μs ({formatLatency(metrics.latency.avg_latency_ns)})</span>
					{:else}
						<span class="not-achieved">⚠️ < 10μs ({formatLatency(metrics.latency.avg_latency_ns)})</span>
					{/if}
				</span>
			</div>
			<div class="target-item">
				<span class="target-label">Cible Throughput:</span>
				<span class="target-value">
					{#if metrics.throughput.messages_per_sec >= 1000000}
						<span class="achieved">✅ 1M+ msg/s ({formatNumber(metrics.throughput.messages_per_sec / 1000000, 2)}M)</span>
					{:else}
						<span class="not-achieved">⚠️ 1M+ msg/s ({formatNumber(metrics.throughput.messages_per_sec / 1000, 0)}K)</span>
					{/if}
				</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.quantum-performance {
		padding: 1.5rem;
		background: linear-gradient(135deg, rgba(20, 20, 30, 0.95) 0%, rgba(30, 30, 45, 0.95) 100%);
		border-radius: 1rem;
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 107, 53, 0.2);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
	}
	
	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 2px solid rgba(255, 107, 53, 0.3);
	}
	
	h2 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #ff6b35;
		margin: 0;
	}
	
	.status-badge .status {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-weight: 600;
		font-size: 0.9rem;
	}
	
	.status.active {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.4);
	}
	
	.status.loading {
		background: rgba(255, 179, 71, 0.2);
		color: #ffb347;
		border: 1px solid rgba(255, 179, 71, 0.4);
	}
	
	.status.error {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.4);
	}
	
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}
	
	.metric-card {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 0.75rem;
		padding: 1.25rem;
		border: 1px solid rgba(255, 107, 53, 0.15);
		transition: all 0.3s ease;
	}
	
	.metric-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(255, 107, 53, 0.2);
		border-color: rgba(255, 107, 53, 0.3);
	}
	
	.metric-card h3 {
		font-size: 1.1rem;
		font-weight: 600;
		margin: 0 0 1rem 0;
		color: #ffb347;
	}
	
	.metric-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}
	
	.metric-row:last-child {
		border-bottom: none;
	}
	
	.metric-row.highlight {
		background: rgba(255, 107, 53, 0.1);
		padding: 0.75rem;
		border-radius: 0.5rem;
		margin: 0.25rem 0;
		border: 1px solid rgba(255, 107, 53, 0.2);
	}
	
	.label {
		color: #94a3b8;
		font-size: 0.9rem;
	}
	
	.value {
		color: #e2e8f0;
		font-weight: 600;
		font-size: 1rem;
	}
	
	.value.positive {
		color: #22c55e;
	}
	
	.value.negative {
		color: #ef4444;
	}
	
	.target-indicators {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		padding: 1rem;
		background: rgba(255, 107, 53, 0.05);
		border-radius: 0.75rem;
		border: 1px dashed rgba(255, 107, 53, 0.3);
	}
	
	.target-item {
		flex: 1;
		min-width: 250px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
	}
	
	.target-label {
		color: #94a3b8;
		font-weight: 600;
	}
	
	.achieved {
		color: #22c55e;
		font-weight: 700;
	}
	
	.not-achieved {
		color: #ffb347;
		font-weight: 700;
	}
	
	@media (max-width: 768px) {
		.metrics-grid {
			grid-template-columns: 1fr;
		}
		
		.target-indicators {
			flex-direction: column;
		}
		
		.target-item {
			min-width: 100%;
		}
	}
</style>
