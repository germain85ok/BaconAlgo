/**
 * BaconAlgo 2040 - Panneau de Contrôle Neural IA
 * Affichage temps réel des signaux AI avec métriques multi-timeframes
 */

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { signalWebSocket, signalsApi } from '$lib/services/apiClient';
	import type { LiveSignal, SignalWithMetrics, Timeframe } from '$lib/types/api';

	// État du composant
	let signals = $state<LiveSignal[]>([]);
	let selectedSignal = $state<SignalWithMetrics | null>(null);
	let connected = $state(false);
	let loading = $state(false);
	let error = $state<string | null>(null);

	// Statistiques IA
	let totalSignals = $state(0);
	let avgConfidence = $state(0);
	let bullishCount = $state(0);
	let bearishCount = $state(0);

	// Timeframes supportés
	const timeframes: Timeframe[] = ['1m', '5m', '15m', '1h', '4h', '1D'];

	// Connexion WebSocket au démarrage
	onMount(() => {
		// Se connecter au WebSocket
		signalWebSocket.connect();
		
		// S'abonner aux signaux
		const unsubscribe = signalWebSocket.onSignal((signal) => {
			// Ajouter le signal au début de la liste
			signals = [signal, ...signals].slice(0, 50); // Garder max 50 signaux
			totalSignals++;
			
			// Mettre à jour les statistiques
			updateStats();
		});

		// Vérifier la connexion
		const checkConnection = setInterval(() => {
			connected = signalWebSocket.isConnected();
		}, 1000);

		return () => {
			unsubscribe();
			signalWebSocket.disconnect();
			clearInterval(checkConnection);
		};
	});

	// Mettre à jour les statistiques
	function updateStats() {
		if (signals.length === 0) return;

		const confidences = signals
			.filter((s) => s.confidence)
			.map((s) => s.confidence!);
		avgConfidence = confidences.length > 0
			? confidences.reduce((sum, c) => sum + c, 0) / confidences.length
			: 0;

		bullishCount = signals.filter((s) => s.direction === 'BUY').length;
		bearishCount = signals.filter((s) => s.direction === 'SELL').length;
	}

	// Sélectionner un signal pour voir les détails
	async function selectSignal(signal: LiveSignal) {
		loading = true;
		error = null;
		
		try {
			const response = await signalsApi.getSignalDetails(signal.id);
			
			if (response.success && response.data) {
				selectedSignal = response.data;
			} else {
				error = response.error || 'Erreur de chargement des détails';
			}
		} catch (e) {
			error = 'Erreur de communication avec le serveur';
		} finally {
			loading = false;
		}
	}

	// Obtenir la couleur en fonction de la tendance
	function getTrendColor(trend: string): string {
		switch (trend) {
			case 'BULLISH':
				return 'text-green-400';
			case 'BEARISH':
				return 'text-red-400';
			default:
				return 'text-gray-400';
		}
	}

	// Obtenir la couleur de fond en fonction de la tendance
	function getTrendBgColor(trend: string): string {
		switch (trend) {
			case 'BULLISH':
				return 'bg-green-500/20 border-green-500/30';
			case 'BEARISH':
				return 'bg-red-500/20 border-red-500/30';
			default:
				return 'bg-gray-500/20 border-gray-500/30';
		}
	}

	// Formater la confiance en pourcentage
	function formatConfidence(conf?: number): string {
		return conf ? `${conf.toFixed(1)}%` : 'N/A';
	}
</script>

<div class="neural-control-panel">
	<!-- Header avec statut de connexion -->
	<div class="panel-header">
		<div class="flex items-center gap-4">
			<div class="header-icon">🧠</div>
			<div>
				<h2 class="header-title">Panneau de Contrôle Neural IA</h2>
				<p class="header-subtitle">Analyse temps réel multi-timeframes</p>
			</div>
		</div>
		<div class="connection-status">
			<div class="status-indicator {connected ? 'connected' : 'disconnected'}"></div>
			<span class="status-text">{connected ? 'Connecté' : 'Déconnecté'}</span>
		</div>
	</div>

	<!-- Statistiques globales -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon">📊</div>
			<div>
				<div class="stat-value">{totalSignals}</div>
				<div class="stat-label">Total Signaux</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon">🎯</div>
			<div>
				<div class="stat-value">{avgConfidence.toFixed(1)}%</div>
				<div class="stat-label">Confiance Moyenne</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon">📈</div>
			<div>
				<div class="stat-value text-green-400">{bullishCount}</div>
				<div class="stat-label">Haussier</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon">📉</div>
			<div>
				<div class="stat-value text-red-400">{bearishCount}</div>
				<div class="stat-label">Baissier</div>
			</div>
		</div>
	</div>

	<div class="panel-content">
		<!-- Liste des signaux -->
		<div class="signals-list">
			<h3 class="section-title">Signaux Actifs</h3>
			
			{#if signals.length === 0}
				<div class="empty-state">
					<div class="empty-icon">⏳</div>
					<p>En attente de signaux...</p>
				</div>
			{:else}
				<div class="signals-scroll">
					{#each signals as signal}
						<button
							class="signal-card {selectedSignal?.signal.id === signal.id ? 'selected' : ''}"
							onclick={() => selectSignal(signal)}
						>
							<div class="signal-header">
								<div class="signal-symbol">{signal.symbol}</div>
								<div class="signal-badge {signal.direction === 'BUY' ? 'bullish' : 'bearish'}">
									{signal.direction || '?'}
								</div>
							</div>
							
							<div class="signal-meta">
								<span class="meta-item">
									<span class="meta-label">Horizon:</span>
									<span class="meta-value">{signal.horizon}</span>
								</span>
								<span class="meta-item">
									<span class="meta-label">Confiance:</span>
									<span class="meta-value">{formatConfidence(signal.confidence)}</span>
								</span>
							</div>

							<!-- Barre de confiance -->
							{#if signal.confidence}
								<div class="confidence-bar">
									<div
										class="confidence-fill {signal.direction === 'BUY' ? 'bullish' : 'bearish'}"
										style="width: {signal.confidence}%"
									></div>
								</div>
							{/if}

							<!-- Tags -->
							<div class="signal-tags">
								{#if signal.tags.near_npoc}
									<span class="tag">NPOC</span>
								{/if}
								{#if signal.tags.near_avwap}
									<span class="tag">AVWAP</span>
								{/if}
								{#if signal.tags.in_golden_pocket}
									<span class="tag gold">Golden Pocket</span>
								{/if}
								{#if signal.tags.structure}
									<span class="tag">Structure: {signal.tags.structure}</span>
								{/if}
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Détails du signal sélectionné -->
		<div class="signal-details">
			{#if loading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Chargement des détails...</p>
				</div>
			{:else if error}
				<div class="error-state">
					<div class="error-icon">⚠️</div>
					<p>{error}</p>
				</div>
			{:else if selectedSignal}
				<h3 class="section-title">Analyse Détaillée</h3>
				
				<!-- Infos du signal -->
				<div class="details-card">
					<h4 class="details-title">Signal: {selectedSignal.signal.symbol}</h4>
					<div class="details-grid">
						<div class="detail-item">
							<span class="detail-label">Direction:</span>
							<span class="detail-value {selectedSignal.signal.direction === 'BUY' ? 'text-green-400' : 'text-red-400'}">
								{selectedSignal.signal.direction}
							</span>
						</div>
						<div class="detail-item">
							<span class="detail-label">Confiance:</span>
							<span class="detail-value">{formatConfidence(selectedSignal.signal.confidence)}</span>
						</div>
						<div class="detail-item">
							<span class="detail-label">Horizon:</span>
							<span class="detail-value">{selectedSignal.signal.horizon}</span>
						</div>
						<div class="detail-item">
							<span class="detail-label">Statut:</span>
							<span class="detail-value">{selectedSignal.signal.ready ? '✅ Prêt' : '⏳ En attente'}</span>
						</div>
					</div>
				</div>

				<!-- Analyse Multi-Timeframes -->
				<div class="details-card">
					<h4 class="details-title">📊 Analyse Multi-Timeframes</h4>
					<div class="timeframes-grid">
						{#each selectedSignal.timeframes as tf}
							<div class="timeframe-card {getTrendBgColor(tf.trend)}">
								<div class="timeframe-label">{tf.timeframe}</div>
								<div class="timeframe-trend {getTrendColor(tf.trend)}">{tf.trend}</div>
								<div class="strength-bar">
									<div class="strength-fill" style="width: {tf.strength}%"></div>
								</div>
								<div class="timeframe-strength">{tf.strength.toFixed(0)}%</div>
							</div>
						{/each}
					</div>
				</div>

				<!-- Indicateurs Leading/Lagging -->
				<div class="indicators-section">
					<!-- Leading Indicators -->
					<div class="details-card">
						<h4 class="details-title">⚡ Indicateurs Leading</h4>
						<div class="indicators-list">
							{#if selectedSignal.indicators.leading.rsi !== undefined}
								<div class="indicator-item">
									<span class="indicator-label">RSI:</span>
									<span class="indicator-value">{selectedSignal.indicators.leading.rsi.toFixed(2)}</span>
								</div>
							{/if}
							{#if selectedSignal.indicators.leading.stochastic !== undefined}
								<div class="indicator-item">
									<span class="indicator-label">Stochastic:</span>
									<span class="indicator-value">{selectedSignal.indicators.leading.stochastic.toFixed(2)}</span>
								</div>
							{/if}
							{#if selectedSignal.indicators.leading.macd_signal}
								<div class="indicator-item">
									<span class="indicator-label">MACD:</span>
									<span class="indicator-value">{selectedSignal.indicators.leading.macd_signal}</span>
								</div>
							{/if}
						</div>
					</div>

					<!-- Lagging Indicators -->
					<div class="details-card">
						<h4 class="details-title">📉 Indicateurs Lagging</h4>
						<div class="indicators-list">
							{#if selectedSignal.indicators.lagging.ma_50 !== undefined}
								<div class="indicator-item">
									<span class="indicator-label">MA 50:</span>
									<span class="indicator-value">{selectedSignal.indicators.lagging.ma_50.toFixed(2)}</span>
								</div>
							{/if}
							{#if selectedSignal.indicators.lagging.ma_200 !== undefined}
								<div class="indicator-item">
									<span class="indicator-label">MA 200:</span>
									<span class="indicator-value">{selectedSignal.indicators.lagging.ma_200.toFixed(2)}</span>
								</div>
							{/if}
							{#if selectedSignal.indicators.lagging.ema_21 !== undefined}
								<div class="indicator-item">
									<span class="indicator-label">EMA 21:</span>
									<span class="indicator-value">{selectedSignal.indicators.lagging.ema_21.toFixed(2)}</span>
								</div>
							{/if}
						</div>
					</div>
				</div>
			{:else}
				<div class="empty-state">
					<div class="empty-icon">🎯</div>
					<p>Sélectionnez un signal pour voir les détails</p>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.neural-control-panel {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		padding: 1.5rem;
		background: rgba(10, 10, 15, 0.6);
		backdrop-filter: blur(20px);
		border-radius: 20px;
		border: 1px solid rgba(255, 107, 53, 0.2);
		box-shadow: 0 8px 32px rgba(255, 107, 53, 0.1);
	}

	/* Header */
	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		background: rgba(255, 107, 53, 0.05);
		border-radius: 16px;
		border: 1px solid rgba(255, 107, 53, 0.2);
	}

	.header-icon {
		font-size: 2.5rem;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.8;
			transform: scale(1.05);
		}
	}

	.header-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ff6b35;
		font-family: 'Orbitron', sans-serif;
	}

	.header-subtitle {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		margin-top: 0.25rem;
	}

	.connection-status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
	}

	.status-indicator {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		animation: blink 2s ease-in-out infinite;
	}

	.status-indicator.connected {
		background: #22c55e;
		box-shadow: 0 0 10px rgba(34, 197, 94, 0.5);
	}

	.status-indicator.disconnected {
		background: #ef4444;
		box-shadow: 0 0 10px rgba(239, 68, 68, 0.5);
	}

	@keyframes blink {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.status-text {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.8);
		font-weight: 600;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(255, 255, 255, 0.03);
		backdrop-filter: blur(10px);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		transition: all 0.3s ease;
	}

	.stat-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 107, 53, 0.3);
		transform: translateY(-2px);
	}

	.stat-icon {
		font-size: 2rem;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #ff6b35;
	}

	.stat-label {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	/* Panel Content */
	.panel-content {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	.signals-list,
	.signal-details {
		background: rgba(255, 255, 255, 0.02);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1.5rem;
		max-height: 700px;
		overflow-y: auto;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #ff6b35;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 2px solid rgba(255, 107, 53, 0.2);
	}

	/* Signals List */
	.signals-scroll {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.signal-card {
		width: 100%;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.3s ease;
		text-align: left;
	}

	.signal-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 107, 53, 0.3);
		transform: translateX(4px);
	}

	.signal-card.selected {
		background: rgba(255, 107, 53, 0.1);
		border-color: rgba(255, 107, 53, 0.5);
		box-shadow: 0 0 20px rgba(255, 107, 53, 0.2);
	}

	.signal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.signal-symbol {
		font-size: 1.125rem;
		font-weight: 700;
		color: white;
	}

	.signal-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.signal-badge.bullish {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.signal-badge.bearish {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.signal-meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 0.5rem;
		font-size: 0.75rem;
	}

	.meta-label {
		color: rgba(255, 255, 255, 0.5);
	}

	.meta-value {
		color: rgba(255, 255, 255, 0.9);
		font-weight: 600;
	}

	/* Confidence Bar */
	.confidence-bar {
		height: 4px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: 0.75rem;
	}

	.confidence-fill {
		height: 100%;
		transition: width 0.5s ease;
	}

	.confidence-fill.bullish {
		background: linear-gradient(90deg, #22c55e, #10b981);
	}

	.confidence-fill.bearish {
		background: linear-gradient(90deg, #ef4444, #dc2626);
	}

	/* Tags */
	.signal-tags {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.tag {
		padding: 0.25rem 0.5rem;
		background: rgba(255, 107, 53, 0.15);
		border: 1px solid rgba(255, 107, 53, 0.3);
		border-radius: 4px;
		font-size: 0.625rem;
		color: #ff6b35;
		font-weight: 600;
		text-transform: uppercase;
	}

	.tag.gold {
		background: rgba(234, 179, 8, 0.15);
		border-color: rgba(234, 179, 8, 0.3);
		color: #eab308;
	}

	/* Details Section */
	.details-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1rem;
	}

	.details-title {
		font-size: 1rem;
		font-weight: 600;
		color: #ff6b35;
		margin-bottom: 0.75rem;
	}

	.details-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.detail-item {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 4px;
	}

	.detail-label {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.detail-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
	}

	/* Timeframes Grid */
	.timeframes-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
	}

	.timeframe-card {
		padding: 0.75rem;
		border-radius: 8px;
		border: 1px solid;
		text-align: center;
	}

	.timeframe-label {
		font-size: 0.75rem;
		font-weight: 700;
		color: rgba(255, 255, 255, 0.8);
		margin-bottom: 0.25rem;
	}

	.timeframe-trend {
		font-size: 0.875rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
	}

	.strength-bar {
		height: 4px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: 0.25rem;
	}

	.strength-fill {
		height: 100%;
		background: linear-gradient(90deg, #ff6b35, #f7931e);
		transition: width 0.5s ease;
	}

	.timeframe-strength {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.7);
	}

	/* Indicators */
	.indicators-section {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.indicators-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.indicator-item {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 4px;
		font-size: 0.875rem;
	}

	.indicator-label {
		color: rgba(255, 255, 255, 0.6);
	}

	.indicator-value {
		color: white;
		font-weight: 600;
	}

	/* Empty and Loading States */
	.empty-state,
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: rgba(255, 255, 255, 0.5);
		text-align: center;
	}

	.empty-icon,
	.error-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(255, 107, 53, 0.2);
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

	/* Responsive */
	@media (max-width: 1024px) {
		.panel-content {
			grid-template-columns: 1fr;
		}

		.indicators-section {
			grid-template-columns: 1fr;
		}

		.timeframes-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr 1fr;
		}

		.panel-header {
			flex-direction: column;
			gap: 1rem;
		}

		.details-grid {
			grid-template-columns: 1fr;
		}

		.timeframes-grid {
			grid-template-columns: 1fr;
		}
	}

	/* Custom Scrollbar */
	.signals-scroll::-webkit-scrollbar,
	.signals-list::-webkit-scrollbar,
	.signal-details::-webkit-scrollbar {
		width: 6px;
	}

	.signals-scroll::-webkit-scrollbar-track,
	.signals-list::-webkit-scrollbar-track,
	.signal-details::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.signals-scroll::-webkit-scrollbar-thumb,
	.signals-list::-webkit-scrollbar-thumb,
	.signal-details::-webkit-scrollbar-thumb {
		background: rgba(255, 107, 53, 0.5);
		border-radius: 3px;
	}

	.signals-scroll::-webkit-scrollbar-thumb:hover,
	.signals-list::-webkit-scrollbar-thumb:hover,
	.signal-details::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 107, 53, 0.7);
	}
</style>
