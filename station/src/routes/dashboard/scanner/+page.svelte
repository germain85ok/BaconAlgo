<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import SignalCard from '$lib/components/dashboard/SignalCard.svelte';
	import FilterSidebar from '$lib/components/dashboard/FilterSidebar.svelte';
	import type { Signal } from '$lib/components/dashboard/SignalCard.svelte';
	import { getSupabaseClient } from '$lib/supabase/client';
	import type { RealtimeChannel } from '@supabase/supabase-js';

	interface Filters {
		horizon: 'all' | 'Scalp' | 'Day' | 'Swing' | 'Long';
		direction: 'all' | 'LONG' | 'SHORT';
		minScore: number;
		minRR: number;
		assetTypes: {
			stocks: boolean;
			crypto: boolean;
			forex: boolean;
			futures: boolean;
		};
		smcFilters: {
			nearFVG: boolean;
			nearOB: boolean;
			bosConfirmed: boolean;
			choch: boolean;
			liquidity: boolean;
			imbalance: boolean;
		};
	}

	let signals = $state<Signal[]>([]);
	let watchlist = $state<Signal[]>([]);
	let loading = $state(true);
	let watchlistOpen = $state(false);
	let sortBy = $state<'time' | 'score' | 'rr'>('time');
	let sortOrder = $state<'asc' | 'desc'>('desc');

	let filters = $state<Filters>({
		horizon: 'all',
		direction: 'all',
		minScore: 0,
		minRR: 0,
		assetTypes: {
			stocks: true,
			crypto: true,
			forex: true,
			futures: true
		},
		smcFilters: {
			nearFVG: false,
			nearOB: false,
			bosConfirmed: false,
			choch: false,
			liquidity: false,
			imbalance: false
		}
	});

	let realtimeChannel: RealtimeChannel | null = null;

	onMount(async () => {
		const supabase = getSupabaseClient();

		// Load signals
		try {
			const { data, error } = await supabase
				.from('signals')
				.select('*')
				.order('created_at', { ascending: false })
				.limit(500);

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

		// Subscribe to real-time
		realtimeChannel = supabase
			.channel('scanner-signals')
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
					];
				}
			)
			.subscribe();
	});

	onDestroy(() => {
		if (realtimeChannel) {
			realtimeChannel.unsubscribe();
		}
	});

	const filteredSignals = $derived.by(() => {
		let result = signals;

		if (filters.direction !== 'all') {
			result = result.filter((s) => s.direction === filters.direction);
		}

		if (filters.horizon !== 'all') {
			result = result.filter((s) => s.horizon === filters.horizon);
		}

		result = result.filter((s) => s.score >= filters.minScore);
		result = result.filter((s) => s.risk_reward_ratio >= filters.minRR);

		// Sort
		result = [...result].sort((a, b) => {
			let comparison = 0;
			if (sortBy === 'score') {
				comparison = a.score - b.score;
			} else if (sortBy === 'rr') {
				comparison = a.risk_reward_ratio - b.risk_reward_ratio;
			} else {
				const aTime = new Date(a.timestamp || '').getTime();
				const bTime = new Date(b.timestamp || '').getTime();
				comparison = aTime - bTime;
			}
			return sortOrder === 'asc' ? comparison : -comparison;
		});

		return result;
	});

	const legendaryCount = $derived(filteredSignals.filter((s) => s.grade === 'S').length);
	const avgScore = $derived(
		filteredSignals.length > 0
			? Math.round(filteredSignals.reduce((a, s) => a + s.score, 0) / filteredSignals.length)
			: 0
	);
	const avgRR = $derived(
		filteredSignals.length > 0
			? (
					filteredSignals.reduce((a, s) => a + s.risk_reward_ratio, 0) / filteredSignals.length
				).toFixed(2)
			: '0.00'
	);

	const handleAddToWatchlist = (signal: Signal) => {
		if (!watchlist.find((s) => s.ticker === signal.ticker)) {
			watchlist = [...watchlist, signal];
		}
	};

	const handleRemoveFromWatchlist = (ticker: string) => {
		watchlist = watchlist.filter((s) => s.ticker !== ticker);
	};

	const handleExportCSV = () => {
		const csv = [
			['Ticker', 'Direction', 'Score', 'Grade', 'Entry', 'Stop Loss', 'TP1', 'R:R', 'Horizon'].join(
				','
			),
			...filteredSignals.map((s) =>
				[
					s.ticker,
					s.direction,
					s.score,
					s.grade,
					s.entry,
					s.stop_loss,
					s.take_profit_1,
					s.risk_reward_ratio,
					s.horizon
				].join(',')
			)
		].join('\n');

		const blob = new Blob([csv], { type: 'text/csv' });
		const url = window.URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `baconalgo-signals-${new Date().toISOString()}.csv`;
		a.click();
	};
</script>

<div class="scanner-dashboard">
	<!-- Plan Badge -->
	<div class="plan-badge-header">
		<div class="badge">
			<span class="badge-icon">🔍</span>
			<span class="badge-text">Scanner Plan - Full Access</span>
		</div>
	</div>

	<!-- Stats Bar -->
	<div class="stats-bar">
		<div class="stat-box">
			<div class="stat-icon">📊</div>
			<div class="stat-content">
				<div class="stat-label">Total Signals</div>
				<div class="stat-value">{filteredSignals.length}</div>
			</div>
		</div>
		<div class="stat-box">
			<div class="stat-icon">🏆</div>
			<div class="stat-content">
				<div class="stat-label">Legendary (S)</div>
				<div class="stat-value">{legendaryCount}</div>
			</div>
		</div>
		<div class="stat-box">
			<div class="stat-icon">⭐</div>
			<div class="stat-content">
				<div class="stat-label">Avg Score</div>
				<div class="stat-value">{avgScore}</div>
			</div>
		</div>
		<div class="stat-box">
			<div class="stat-icon">🎯</div>
			<div class="stat-content">
				<div class="stat-label">Avg R:R</div>
				<div class="stat-value">1:{avgRR}</div>
			</div>
		</div>
	</div>

	<!-- Main Content -->
	<div class="main-layout">
		<!-- Filter Sidebar -->
		<div class="sidebar-container">
			<FilterSidebar bind:filters />
		</div>

		<!-- Signals Area -->
		<div class="signals-area">
			<!-- Toolbar -->
			<div class="toolbar">
				<div class="toolbar-left">
					<button class="watchlist-toggle" onclick={() => (watchlistOpen = !watchlistOpen)}>
						📌 Watchlist ({watchlist.length})
					</button>
				</div>

				<div class="toolbar-right">
					<div class="sort-controls">
						<label for="sortBy">Sort by:</label>
						<select id="sortBy" bind:value={sortBy}>
							<option value="time">Time</option>
							<option value="score">Score</option>
							<option value="rr">R:R</option>
						</select>
						<button
							class="sort-order-btn"
							onclick={() => (sortOrder = sortOrder === 'asc' ? 'desc' : 'asc')}
						>
							{#if sortOrder === 'asc'}
								<svg
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<path d="M12 19V5M5 12l7-7 7 7" />
								</svg>
							{:else}
								<svg
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<path d="M12 5v14M19 12l-7 7-7-7" />
								</svg>
							{/if}
						</button>
					</div>

					<button class="export-btn" onclick={handleExportCSV}>
						<svg
							width="18"
							height="18"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
							<polyline points="7 10 12 15 17 10" />
							<line x1="12" y1="15" x2="12" y2="3" />
						</svg>
						Export CSV
					</button>
				</div>
			</div>

			<!-- Signals Grid -->
			{#if loading}
				<div class="loading">
					<div class="spinner"></div>
					<p>Loading signals...</p>
				</div>
			{:else if filteredSignals.length === 0}
				<div class="empty-state">
					<div class="empty-icon">📊</div>
					<h4>No signals match your filters</h4>
					<p>Try adjusting the filters on the left</p>
				</div>
			{:else}
				<div class="signals-grid">
					{#each filteredSignals as signal}
						<SignalCard {signal} onAddToWatchlist={handleAddToWatchlist} />
					{/each}
				</div>
			{/if}
		</div>

		<!-- Watchlist Panel -->
		{#if watchlistOpen}
			<div class="watchlist-panel">
				<div class="watchlist-header">
					<h3>Watchlist</h3>
					<button class="close-btn" onclick={() => (watchlistOpen = false)}>✕</button>
				</div>

				{#if watchlist.length === 0}
					<div class="watchlist-empty">
						<p>No symbols in watchlist</p>
						<small>Click "Watchlist" on any signal to add it</small>
					</div>
				{:else}
					<div class="watchlist-items">
						{#each watchlist as item}
							<div class="watchlist-item">
								<div class="watchlist-ticker">{item.ticker}</div>
								<div class="watchlist-info">
									<span class="watchlist-score">{item.score}</span>
									<span class="watchlist-direction {item.direction.toLowerCase()}">
										{item.direction}
									</span>
								</div>
								<button
									class="remove-btn"
									onclick={() => handleRemoveFromWatchlist(item.ticker)}
								>
									✕
								</button>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.scanner-dashboard {
		max-width: 1800px;
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
		background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
		border-radius: 9999px;
		color: #1f2937;
		font-weight: 700;
		font-size: 1rem;
	}

	.stats-bar {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-box {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
	}

	.stat-icon {
		font-size: 2rem;
	}

	.stat-content {
		flex: 1;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #9ca3af;
		margin-bottom: 0.25rem;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.main-layout {
		display: grid;
		grid-template-columns: 280px 1fr auto;
		gap: 1.5rem;
	}

	.sidebar-container {
		position: sticky;
		top: 1rem;
		height: fit-content;
	}

	.signals-area {
		flex: 1;
	}

	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		background: rgba(17, 24, 39, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		margin-bottom: 1.5rem;
	}

	.toolbar-left,
	.toolbar-right {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.watchlist-toggle {
		padding: 0.625rem 1rem;
		background: rgba(255, 107, 53, 0.1);
		border: 1px solid rgba(255, 107, 53, 0.3);
		border-radius: 0.5rem;
		color: #ff6b35;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.watchlist-toggle:hover {
		background: rgba(255, 107, 53, 0.2);
		border-color: rgba(255, 107, 53, 0.5);
	}

	.sort-controls {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.sort-controls label {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.sort-controls select {
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.375rem;
		color: #f9fafb;
		font-size: 0.875rem;
	}

	.sort-order-btn {
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.375rem;
		color: #e5e7eb;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.sort-order-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.export-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		border: none;
		border-radius: 0.5rem;
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.export-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(16, 185, 129, 0.4);
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

	.watchlist-panel {
		width: 300px;
		position: sticky;
		top: 1rem;
		height: fit-content;
		max-height: calc(100vh - 2rem);
		background: rgba(17, 24, 39, 0.8);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.watchlist-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.watchlist-header h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 700;
		color: #f9fafb;
	}

	.close-btn {
		background: none;
		border: none;
		color: #9ca3af;
		font-size: 1.25rem;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.close-btn:hover {
		color: #ef4444;
	}

	.watchlist-empty {
		padding: 2rem;
		text-align: center;
		color: #6b7280;
	}

	.watchlist-empty small {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #4b5563;
	}

	.watchlist-items {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.watchlist-items::-webkit-scrollbar {
		width: 6px;
	}

	.watchlist-items::-webkit-scrollbar-track {
		background: rgba(0, 0, 0, 0.2);
	}

	.watchlist-items::-webkit-scrollbar-thumb {
		background: rgba(255, 107, 53, 0.3);
		border-radius: 3px;
	}

	.watchlist-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.875rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.watchlist-ticker {
		font-weight: 700;
		color: #f9fafb;
	}

	.watchlist-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.watchlist-score {
		padding: 0.25rem 0.5rem;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 700;
		color: #fbbf24;
	}

	.watchlist-direction {
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.watchlist-direction.long {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #10b981;
	}

	.watchlist-direction.short {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	.remove-btn {
		background: none;
		border: none;
		color: #6b7280;
		font-size: 1rem;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.remove-btn:hover {
		color: #ef4444;
	}

	@media (max-width: 1400px) {
		.main-layout {
			grid-template-columns: 1fr;
		}

		.sidebar-container {
			position: static;
		}

		.watchlist-panel {
			position: fixed;
			top: 0;
			right: 0;
			height: 100vh;
			z-index: 100;
		}
	}

	@media (max-width: 640px) {
		.toolbar {
			flex-direction: column;
			gap: 1rem;
		}

		.toolbar-left,
		.toolbar-right {
			width: 100%;
			justify-content: space-between;
		}

		.signals-grid {
			grid-template-columns: 1fr;
		}

		.watchlist-panel {
			width: 100%;
		}
	}
</style>
