<script lang="ts">
	import QuantumPerformance from '$lib/components/QuantumPerformance.svelte';
</script>

<svelte:head>
	<title>Performance Quantique - BaconAlgo 2040</title>
</svelte:head>

<div class="performance-page">
	<div class="page-header">
		<h1>🥓⚡ BaconAlgo 2040 Quantum Edition</h1>
		<p class="subtitle">Plateforme de Trading Ultra-Haute Fréquence</p>
		<p class="tagline">Performance de niveau Hedge Fund • Latence < 10μs • Throughput 1M+ msg/s</p>
	</div>
	
	<QuantumPerformance />
	
	<div class="info-section">
		<div class="info-card">
			<h3>🎯 Objectifs de Performance</h3>
			<ul>
				<li><strong>Latence:</strong> < 10 microsecondes</li>
				<li><strong>Scan:</strong> 10,000+ instruments en < 100ms</li>
				<li><strong>Throughput:</strong> 1M+ messages/seconde</li>
				<li><strong>Backtesting:</strong> 1 an de ticks en < 5s</li>
				<li><strong>Uptime:</strong> 99.999%</li>
			</ul>
		</div>
		
		<div class="info-card">
			<h3>⚡ Composants Actifs</h3>
			<ul>
				<li>✅ Quantum Engine (lock-free)</li>
				<li>✅ Smart Order Router</li>
				<li>✅ Risk Manager (VaR, Kelly)</li>
				<li>✅ Quantum Scanner</li>
				<li>✅ Signal Engine (multi-timeframe)</li>
				<li>✅ Market Data Handler</li>
				<li>✅ Quantum Bus (Disruptor)</li>
				<li>✅ Backtest Engine (vectorisé)</li>
			</ul>
		</div>
		
		<div class="info-card">
			<h3>🔧 Optimisations</h3>
			<ul>
				<li>Lock-free data structures (crossbeam)</li>
				<li>Zero-copy serialization (rkyv)</li>
				<li>Mimalloc global allocator</li>
				<li>SIMD operations (AVX2, SSE4.2)</li>
				<li>Thread pinning (CPU affinity)</li>
				<li>Parallel processing (rayon)</li>
				<li>Target CPU native compilation</li>
				<li>Thin LTO link-time optimization</li>
			</ul>
		</div>
	</div>
	
	<div class="api-docs">
		<h3>📡 API Endpoints</h3>
		<div class="endpoint">
			<code>GET /api/metrics</code>
			<span>Métriques de performance en temps réel</span>
		</div>
		<div class="endpoint">
			<code>GET /api/health</code>
			<span>Health check du système</span>
		</div>
		<div class="endpoint">
			<code>GET /api/status</code>
			<span>Status des composants</span>
		</div>
		<div class="endpoint">
			<code>GET /api/signals/stream</code>
			<span>Stream SSE des signaux de trading</span>
		</div>
	</div>
</div>

<style>
	.performance-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
		min-height: 100vh;
		background: linear-gradient(135deg, #0a0a0f 0%, #1a1a2e 100%);
	}
	
	.page-header {
		text-align: center;
		margin-bottom: 3rem;
		padding: 2rem;
		background: linear-gradient(135deg, rgba(255, 107, 53, 0.1) 0%, rgba(255, 179, 71, 0.1) 100%);
		border-radius: 1rem;
		border: 2px solid rgba(255, 107, 53, 0.3);
	}
	
	h1 {
		font-size: 3rem;
		font-weight: 800;
		background: linear-gradient(135deg, #ff6b35 0%, #ffb347 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin: 0 0 0.5rem 0;
	}
	
	.subtitle {
		font-size: 1.5rem;
		color: #94a3b8;
		margin: 0.5rem 0;
	}
	
	.tagline {
		font-size: 1rem;
		color: #64748b;
		margin: 0.5rem 0 0 0;
	}
	
	.info-section {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
		gap: 1.5rem;
		margin: 2rem 0;
	}
	
	.info-card {
		background: rgba(255, 255, 255, 0.03);
		border-radius: 1rem;
		padding: 1.5rem;
		border: 1px solid rgba(255, 107, 53, 0.2);
	}
	
	.info-card h3 {
		color: #ffb347;
		font-size: 1.3rem;
		margin: 0 0 1rem 0;
	}
	
	.info-card ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}
	
	.info-card li {
		color: #cbd5e1;
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
		font-size: 0.95rem;
	}
	
	.info-card li:last-child {
		border-bottom: none;
	}
	
	.api-docs {
		margin-top: 2rem;
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 1rem;
		border: 1px solid rgba(255, 107, 53, 0.2);
	}
	
	.api-docs h3 {
		color: #ffb347;
		font-size: 1.3rem;
		margin: 0 0 1rem 0;
	}
	
	.endpoint {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		margin: 0.5rem 0;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 0.5rem;
		border: 1px solid rgba(255, 107, 53, 0.1);
	}
	
	.endpoint code {
		background: rgba(255, 107, 53, 0.2);
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		color: #ff6b35;
		font-weight: 600;
		font-family: 'Monaco', 'Courier New', monospace;
	}
	
	.endpoint span {
		color: #94a3b8;
		font-size: 0.9rem;
	}
	
	@media (max-width: 768px) {
		.performance-page {
			padding: 1rem;
		}
		
		h1 {
			font-size: 2rem;
		}
		
		.subtitle {
			font-size: 1.2rem;
		}
		
		.info-section {
			grid-template-columns: 1fr;
		}
		
		.endpoint {
			flex-direction: column;
			gap: 0.5rem;
			align-items: flex-start;
		}
	}
</style>
