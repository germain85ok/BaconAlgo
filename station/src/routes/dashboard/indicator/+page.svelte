<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import SignalCard from '$lib/components/dashboard/SignalCard.svelte';
	import type { Signal } from '$lib/components/dashboard/SignalCard.svelte';
	import { getSupabaseClient } from '$lib/supabase/client';
	import type { RealtimeChannel } from '@supabase/supabase-js';

	let signals = $state<Signal[]>([]);
	let loading = $state(true);
	let direction = $state<'all' | 'LONG' | 'SHORT'>('all');
	let minScore = $state(0);
	let apiKey = $state('ba_live_xxxxxxxxxxxxxxxx');
	let discordWebhook = $state('');
	let alertsEnabled = $state(true);
	let copySuccess = $state(false);

	let realtimeChannel: RealtimeChannel | null = null;

	const mockAlerts = [
		{ id: 1, ticker: 'AAPL', type: 'Entry', price: 175.5, time: '2 min ago' },
		{ id: 2, ticker: 'TSLA', type: 'TP1 Hit', price: 235.0, time: '15 min ago' },
		{ id: 3, ticker: 'NVDA', type: 'Entry', price: 720.0, time: '30 min ago' },
		{ id: 4, ticker: 'SPY', type: 'Stop Loss', price: 445.0, time: '1 hour ago' }
	];

	onMount(async () => {
		const supabase = getSupabaseClient();

		// Load signals
		try {
			const { data, error } = await supabase
				.from('signals')
				.select('*')
				.order('created_at', { ascending: false })
				.limit(50);

			if (!error && data) {
				signals = data.map((d: any) => ({
					ticker: d.ticker,
					direction: d.direction,
					score: d.score,
					grade: d.grade,
					entry: d.entry_price || 0,
					stop_loss: d.stop_loss || 0,
					take_profit_1: d.take_profit_1 || 0,
					take_profit_2: d.take_profit_2 || 0,
					take_profit_3: d.take_profit_3 || 0,
					risk_reward_ratio: d.risk_reward_ratio || 0,
					smc_tags: d.smc_tags || [],
					horizon: d.horizon || 'Day',
					timestamp: d.created_at
				}));
			}
		} catch (e) {
			console.error('Error loading signals:', e);
		} finally {
			loading = false;
		}

		// Subscribe to real-time signals
		realtimeChannel = supabase
			.channel('signals-realtime')
			.on(
				'postgres_changes',
				{ event: 'INSERT', schema: 'public', table: 'signals' },
				(payload) => {
					const newSignal = payload.new as any;
					signals = [
						{
							ticker: newSignal.ticker,
							direction: newSignal.direction,
							score: newSignal.score,
							grade: newSignal.grade,
							entry: newSignal.entry_price || 0,
							stop_loss: newSignal.stop_loss || 0,
							take_profit_1: newSignal.take_profit_1 || 0,
							take_profit_2: newSignal.take_profit_2 || 0,
							take_profit_3: newSignal.take_profit_3 || 0,
							risk_reward_ratio: newSignal.risk_reward_ratio || 0,
							smc_tags: newSignal.smc_tags || [],
							horizon: newSignal.horizon || 'Day',
							timestamp: newSignal.created_at
						},
						...signals
					].slice(0, 50);
				}
			)
			.subscribe();
	});

	onDestroy(() => {
		if (realtimeChannel) {
			realtimeChannel.unsubscribe();
		}
	});

	const filteredSignals = $derived(
		signals
			.filter((s) => (direction === 'all' ? true : s.direction === direction))
			.filter((s) => s.score >= minScore)
	);

	const handleCopyApiKey = () => {
		navigator.clipboard.writeText(apiKey);
		copySuccess = true;
		setTimeout(() => (copySuccess = false), 2000);
	};

	const handleSaveDiscord = () => {
		alert('Discord webhook saved!');
	};
</script>

<div class="indicator-dashboard">
	<!-- Plan Badge -->
	<div class="plan-badge-header">
		<div class="badge">
			<span class="badge-icon">📊</span>
			<span class="badge-text">Indicator Plan</span>
		</div>
	</div>

	<!-- Stats Bar -->
	<div class="stats-bar">
		<div class="stat-box">
			<div class="stat-label">Total Signals</div>
			<div class="stat-value">{signals.length}</div>
		</div>
		<div class="stat-box">
			<div class="stat-label">Filtered</div>
			<div class="stat-value">{filteredSignals.length}</div>
		</div>
		<div class="stat-box">
			<div class="stat-label">Avg Score</div>
			<div class="stat-value">
				{filteredSignals.length > 0
					? Math.round(filteredSignals.reduce((a, s) => a + s.score, 0) / filteredSignals.length)
					: 0}
			</div>
		</div>
		<div class="stat-box">
			<div class="stat-label">Real-time</div>
			<div class="stat-value live">
				<span class="live-dot"></span> LIVE
			</div>
		</div>
	</div>

	<!-- TradingView Indicator Section -->
	<div class="indicator-section">
		<div class="section-icon">📈</div>
		<h3>TradingView Indicator Access</h3>
		<p>Use your API key to connect BaconAlgo signals to TradingView</p>

		<div class="api-key-container">
			<div class="api-key-display">
				<code>{apiKey}</code>
				<button class="copy-btn" onclick={handleCopyApiKey}>
					{#if copySuccess}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<polyline points="20 6 9 17 4 12" />
						</svg>
						Copied!
					{:else}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
							<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
						</svg>
						Copy
					{/if}
				</button>
			</div>
		</div>

		<div class="download-links">
			<a href="#" class="download-btn">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
					<polyline points="7 10 12 15 17 10" />
					<line x1="12" y1="15" x2="12" y2="3" />
				</svg>
				Download Indicator (.pine)
			</a>
			<a href="#" class="docs-link">📖 Setup Guide</a>
		</div>
	</div>

	<!-- Discord Alerts Section -->
	<div class="discord-section">
		<div class="section-icon">🔔</div>
		<h3>Discord Alert Configuration</h3>
		<p>Receive instant signal alerts in your Discord server</p>

		<div class="discord-form">
			<div class="form-group">
				<label for="webhook">Discord Webhook URL</label>
				<input
					id="webhook"
					type="text"
					placeholder="https://discord.com/api/webhooks/..."
					bind:value={discordWebhook}
				/>
			</div>

			<div class="form-group">
				<label class="toggle-label">
					<input type="checkbox" bind:checked={alertsEnabled} />
					<span class="toggle-text">Enable Discord Alerts</span>
				</label>
			</div>

			<button class="save-btn" onclick={handleSaveDiscord}>Save Configuration</button>
		</div>
	</div>

	<!-- Filters -->
	<div class="filter-bar">
		<div class="filter-group">
			<label for="dir">Direction</label>
			<select id="dir" bind:value={direction}>
				<option value="all">Both</option>
				<option value="LONG">Long</option>
				<option value="SHORT">Short</option>
			</select>
		</div>

		<div class="filter-group">
			<label for="score">Min Score: {minScore}</label>
			<input id="score" type="range" min="0" max="100" step="5" bind:value={minScore} />
		</div>
	</div>

	<!-- Signals Grid -->
	<div class="signals-section">
		<h3>Real-Time Signals (Up to 50)</h3>

		{#if loading}
			<div class="loading">
				<div class="spinner"></div>
				<p>Loading signals...</p>
			</div>
		{:else if filteredSignals.length === 0}
			<div class="empty-state">
				<div class="empty-icon">📊</div>
				<h4>No signals match your filters</h4>
				<p>Try adjusting the filters above</p>
			</div>
		{:else}
			<div class="signals-grid">
				{#each filteredSignals as signal}
					<SignalCard {signal} />
				{/each}
			</div>
		{/if}
	</div>

	<!-- Alert History -->
	<div class="alert-history">
		<h3>Recent Alerts (Last 20)</h3>
		<div class="alerts-table">
			<table>
				<thead>
					<tr>
						<th>Ticker</th>
						<th>Type</th>
						<th>Price</th>
						<th>Time</th>
					</tr>
				</thead>
				<tbody>
					{#each mockAlerts as alert}
						<tr>
							<td class="ticker-cell">{alert.ticker}</td>
							<td>
								<span class="alert-type">{alert.type}</span>
							</td>
							<td>${alert.price.toFixed(2)}</td>
							<td class="time-cell">{alert.time}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>

<style>
	.indicator-dashboard {
		max-width: 1400px;
		margin: 0 auto;
	}

	.plan-badge-header {
		margin-bottom: 1.5rem;
	}

	.badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #3b82f6 0%, #06b6d4 100%);
		border-radius: 9999px;
		color: #fff;
		font-weight: 700;
		font-size: 1rem;
	}

	.stats-bar {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-box {
		padding: 1.25rem;
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.5rem;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.stat-value.live {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #10b981;
		font-size: 1.25rem;
	}

	.live-dot {
		width: 10px;
		height: 10px;
		background: #10b981;
		border-radius: 50%;
		animation: pulse 2s infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}

	.indicator-section,
	.discord-section {
		padding: 2rem;
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		margin-bottom: 2rem;
		text-align: center;
	}

	.section-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.indicator-section h3,
	.discord-section h3 {
		margin: 0 0 0.5rem 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.indicator-section p,
	.discord-section p {
		margin: 0 0 1.5rem 0;
		color: #9ca3af;
	}

	.api-key-container {
		max-width: 600px;
		margin: 0 auto 1.5rem;
	}

	.api-key-display {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
	}

	.api-key-display code {
		flex: 1;
		color: #fbbf24;
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.875rem;
	}

	.copy-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(255, 107, 53, 0.2);
		border: 1px solid rgba(255, 107, 53, 0.3);
		border-radius: 0.375rem;
		color: #ff6b35;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.copy-btn:hover {
		background: rgba(255, 107, 53, 0.3);
		border-color: rgba(255, 107, 53, 0.5);
	}

	.download-links {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
	}

	.download-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		border-radius: 0.5rem;
		color: #1f2937;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.download-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(255, 107, 53, 0.4);
	}

	.docs-link {
		color: #3b82f6;
		font-weight: 600;
		text-decoration: none;
	}

	.docs-link:hover {
		text-decoration: underline;
	}

	.discord-form {
		max-width: 600px;
		margin: 0 auto;
		text-align: left;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #e5e7eb;
	}

	.form-group input[type='text'] {
		width: 100%;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #f9fafb;
		font-size: 0.875rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.toggle-label input[type='checkbox'] {
		width: 1.25rem;
		height: 1.25rem;
		cursor: pointer;
		accent-color: #ff6b35;
	}

	.save-btn {
		width: 100%;
		padding: 0.875rem;
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		border: none;
		border-radius: 0.5rem;
		color: #fff;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.save-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(16, 185, 129, 0.4);
	}

	.filter-bar {
		display: flex;
		gap: 1.5rem;
		padding: 1.5rem;
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		margin-bottom: 2rem;
	}

	.filter-group {
		flex: 1;
	}

	.filter-group label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #e5e7eb;
	}

	.filter-group select,
	.filter-group input[type='range'] {
		width: 100%;
		padding: 0.625rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #f9fafb;
	}

	.signals-section {
		margin-bottom: 2rem;
	}

	.signals-section h3 {
		margin: 0 0 1.5rem 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f9fafb;
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

	.alert-history h3 {
		margin: 0 0 1rem 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.alerts-table {
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		overflow: hidden;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	thead {
		background: rgba(0, 0, 0, 0.3);
	}

	th {
		padding: 1rem;
		text-align: left;
		font-size: 0.875rem;
		font-weight: 600;
		color: #9ca3af;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	td {
		padding: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
		color: #e5e7eb;
	}

	.ticker-cell {
		font-weight: 700;
		color: #f9fafb;
	}

	.alert-type {
		padding: 0.25rem 0.625rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #3b82f6;
	}

	.time-cell {
		color: #9ca3af;
		font-size: 0.875rem;
	}

	@media (max-width: 640px) {
		.filter-bar {
			flex-direction: column;
		}

		.signals-grid {
			grid-template-columns: 1fr;
		}

		.download-links {
			flex-direction: column;
		}
	}
</style>
