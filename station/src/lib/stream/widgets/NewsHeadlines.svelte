<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';

	interface NewsItem {
		id: string;
		headline: string;
		source: string;
		timestamp: Date;
	}

	let headlines: NewsItem[] = [
		{
			id: '1',
			headline: 'Federal Reserve signals potential rate changes amid economic shifts',
			source: 'Bloomberg',
			timestamp: new Date(Date.now() - 5 * 60000)
		},
		{
			id: '2',
			headline: 'Technology sector shows strong quarterly earnings performance',
			source: 'CNBC',
			timestamp: new Date(Date.now() - 15 * 60000)
		},
		{
			id: '3',
			headline: 'Cryptocurrency markets experience increased institutional adoption',
			source: 'CoinDesk',
			timestamp: new Date(Date.now() - 25 * 60000)
		},
		{
			id: '4',
			headline: 'Energy sector responds to global production adjustments',
			source: 'Reuters',
			timestamp: new Date(Date.now() - 35 * 60000)
		},
		{
			id: '5',
			headline: 'Housing market indicators point to evolving trends',
			source: 'WSJ',
			timestamp: new Date(Date.now() - 45 * 60000)
		},
		{
			id: '6',
			headline: 'Artificial intelligence stocks maintain investor interest',
			source: 'MarketWatch',
			timestamp: new Date(Date.now() - 55 * 60000)
		}
	];

	let currentIndex = 0;
	let interval: ReturnType<typeof setInterval>;

	function formatTimeAgo(date: Date): string {
		const seconds = Math.floor((Date.now() - date.getTime()) / 1000);
		if (seconds < 60) return `${seconds}s ago`;
		const minutes = Math.floor(seconds / 60);
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		return `${hours}h ago`;
	}

	function cycleHeadlines() {
		currentIndex = (currentIndex + 1) % headlines.length;
	}

	onMount(() => {
		interval = setInterval(cycleHeadlines, 8000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});

	$: visibleHeadlines = [
		headlines[currentIndex],
		headlines[(currentIndex + 1) % headlines.length],
		headlines[(currentIndex + 2) % headlines.length]
	];
</script>

<div class="news-headlines">
	<div class="header">
		<h3>
			<svg
				width="16"
				height="16"
				viewBox="0 0 16 16"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<circle cx="8" cy="8" r="3" fill="currentColor" />
				<circle cx="8" cy="8" r="5" stroke="currentColor" stroke-width="1" opacity="0.5" />
			</svg>
			Breaking News
		</h3>
		<div class="live-badge">LIVE</div>
	</div>

	<div class="headlines-container">
		{#each visibleHeadlines as headline (headline.id)}
			<div class="headline-item" in:fade={{ duration: 300 }}>
				<div class="headline-text">{headline.headline}</div>
				<div class="headline-meta">
					<span class="source">{headline.source}</span>
					<span class="timestamp">{formatTimeAgo(headline.timestamp)}</span>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.news-headlines {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 350px;
		max-height: 300px;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	h3 svg {
		color: #ef4444;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.live-badge {
		background: #ef4444;
		color: white;
		font-size: 0.625rem;
		font-weight: 700;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		letter-spacing: 0.05em;
		animation: pulse 2s ease-in-out infinite;
	}

	.headlines-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		overflow: hidden;
	}

	.headline-item {
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border-left: 3px solid #f97316;
		border-radius: 4px;
		transition: all 0.3s ease;
	}

	.headline-item:hover {
		background: rgba(255, 255, 255, 0.08);
		border-left-color: #ff6b35;
	}

	.headline-text {
		font-size: 0.875rem;
		line-height: 1.4;
		color: rgba(255, 255, 255, 0.9);
		margin-bottom: 0.5rem;
	}

	.headline-meta {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.75rem;
	}

	.source {
		color: #f97316;
		font-weight: 600;
	}

	.timestamp {
		color: rgba(255, 255, 255, 0.5);
	}
</style>
