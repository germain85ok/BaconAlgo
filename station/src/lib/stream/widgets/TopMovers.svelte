<script lang="ts">
	import { topMovers } from '$lib/stores/market';
	import type { TopMover } from '$lib/stores/market';

	$: gainers = $topMovers
		.filter((m) => m.changePercent > 0)
		.sort((a, b) => b.changePercent - a.changePercent)
		.slice(0, 5);

	$: losers = $topMovers
		.filter((m) => m.changePercent < 0)
		.sort((a, b) => a.changePercent - b.changePercent)
		.slice(0, 5);

	function formatVolume(vol: number): string {
		if (vol >= 1000000000) return `${(vol / 1000000000).toFixed(1)}B`;
		if (vol >= 1000000) return `${(vol / 1000000).toFixed(1)}M`;
		if (vol >= 1000) return `${(vol / 1000).toFixed(1)}K`;
		return vol.toString();
	}
</script>

<div class="top-movers">
	<h3>Top Movers</h3>

	<div class="movers-grid">
		<div class="movers-column">
			<div class="column-header gainers">
				<svg
					width="16"
					height="16"
					viewBox="0 0 16 16"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path d="M8 3L13 11H3L8 3Z" fill="currentColor" />
				</svg>
				Top Gainers
			</div>
			<div class="movers-list">
				{#each gainers as mover (mover.ticker)}
					<div class="mover-item">
						<div class="mover-info">
							<span class="ticker">{mover.ticker}</span>
							<span class="volume">{formatVolume(mover.volume)}</span>
						</div>
						<div class="mover-stats">
							<span class="price">${mover.price.toFixed(2)}</span>
							<span class="change positive">
								+{mover.changePercent.toFixed(2)}%
							</span>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<div class="movers-column">
			<div class="column-header losers">
				<svg
					width="16"
					height="16"
					viewBox="0 0 16 16"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path d="M8 13L13 5H3L8 13Z" fill="currentColor" />
				</svg>
				Top Losers
			</div>
			<div class="movers-list">
				{#each losers as mover (mover.ticker)}
					<div class="mover-item">
						<div class="mover-info">
							<span class="ticker">{mover.ticker}</span>
							<span class="volume">{formatVolume(mover.volume)}</span>
						</div>
						<div class="mover-stats">
							<span class="price">${mover.price.toFixed(2)}</span>
							<span class="change negative">
								{mover.changePercent.toFixed(2)}%
							</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>

<style>
	.top-movers {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 450px;
	}

	h3 {
		margin: 0 0 1rem 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.movers-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	.column-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 0.75rem;
		padding-bottom: 0.5rem;
		border-bottom: 2px solid;
	}

	.column-header.gainers {
		color: #10b981;
		border-color: #10b981;
	}

	.column-header.losers {
		color: #ef4444;
		border-color: #ef4444;
	}

	.movers-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.mover-item {
		background: rgba(255, 255, 255, 0.03);
		border-radius: 6px;
		padding: 0.75rem;
		transition: all 0.3s ease;
	}

	.mover-item:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateX(4px);
	}

	.mover-info {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.25rem;
	}

	.ticker {
		font-weight: 700;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.9);
	}

	.volume {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.mover-stats {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.price {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.7);
	}

	.change {
		font-size: 0.875rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.change.positive {
		color: #10b981;
		background: rgba(16, 185, 129, 0.1);
	}

	.change.negative {
		color: #ef4444;
		background: rgba(239, 68, 68, 0.1);
	}

	@media (max-width: 768px) {
		.movers-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
