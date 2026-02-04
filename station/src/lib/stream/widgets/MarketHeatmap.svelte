<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	interface Sector {
		name: string;
		symbol: string;
		change: number;
	}

	let sectors: Sector[] = [
		{ name: 'Tech', symbol: 'XLK', change: 0 },
		{ name: 'Finance', symbol: 'XLF', change: 0 },
		{ name: 'Health', symbol: 'XLV', change: 0 },
		{ name: 'Energy', symbol: 'XLE', change: 0 },
		{ name: 'BTC', symbol: 'BTC', change: 0 },
		{ name: 'ETH', symbol: 'ETH', change: 0 },
		{ name: 'SOL', symbol: 'SOL', change: 0 },
		{ name: 'AVAX', symbol: 'AVAX', change: 0 },
		{ name: 'Retail', symbol: 'XRT', change: 0 },
		{ name: 'Real Estate', symbol: 'XLRE', change: 0 },
		{ name: 'Utils', symbol: 'XLU', change: 0 },
		{ name: 'Materials', symbol: 'XLB', change: 0 }
	];

	let interval: ReturnType<typeof setInterval>;

	function getColor(change: number): string {
		if (change > 3) return '#10b981';
		if (change > 1) return '#22c55e';
		if (change > 0) return '#4ade80';
		if (change > -1) return '#ef4444';
		if (change > -3) return '#dc2626';
		return '#b91c1c';
	}

	function updateMockData() {
		sectors = sectors.map((sector) => ({
			...sector,
			change: (Math.random() - 0.5) * 8
		}));
	}

	onMount(() => {
		updateMockData();
		interval = setInterval(updateMockData, 5000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});
</script>

<div class="heatmap">
	<h3>Market Heatmap</h3>
	<div class="grid">
		{#each sectors as sector (sector.symbol)}
			<div
				class="sector-card"
				style="background: {getColor(sector.change)}; 
					   box-shadow: 0 0 20px {getColor(sector.change)}33;"
			>
				<div class="sector-name">{sector.name}</div>
				<div class="sector-symbol">{sector.symbol}</div>
				<div class="sector-change">
					{sector.change > 0 ? '+' : ''}{sector.change.toFixed(2)}%
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.heatmap {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 400px;
	}

	h3 {
		margin: 0 0 1rem 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
	}

	.sector-card {
		aspect-ratio: 1;
		border-radius: 8px;
		padding: 0.75rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		transition: all 0.3s ease;
		cursor: pointer;
		position: relative;
		overflow: hidden;
	}

	.sector-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
		transition: left 0.5s;
	}

	.sector-card:hover::before {
		left: 100%;
	}

	.sector-card:hover {
		transform: scale(1.05);
	}

	.sector-name {
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
	}

	.sector-symbol {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.8);
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
	}

	.sector-change {
		font-size: 1rem;
		font-weight: 700;
		color: white;
		text-align: right;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
	}

	@media (max-width: 768px) {
		.grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
