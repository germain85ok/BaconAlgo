<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	const barCount = 20;
	let bars: number[] = Array(barCount).fill(0);
	let interval: ReturnType<typeof setInterval>;

	function updateBars() {
		bars = bars.map(() => Math.random() * 100);
	}

	onMount(() => {
		updateBars();
		interval = setInterval(updateBars, 100);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});
</script>

<div class="music-visualizer">
	<div class="bars-container">
		{#each bars as height, i (i)}
			<div
				class="bar"
				style="height: {height}%; 
					   animation-delay: {i * 0.05}s;
					   background: linear-gradient(to top, 
						   hsl({(i / barCount) * 60 + 10}, 100%, 50%),
						   hsl({(i / barCount) * 60 + 40}, 100%, 60%));"
			/>
		{/each}
	</div>
</div>

<style>
	.music-visualizer {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 300px;
		overflow: hidden;
	}

	.bars-container {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		height: 120px;
		gap: 4px;
	}

	.bar {
		flex: 1;
		min-height: 8px;
		border-radius: 4px 4px 0 0;
		transition: height 0.1s ease-out;
		box-shadow: 0 0 10px currentColor;
		animation: pulse 0.6s ease-in-out infinite alternate;
	}

	@keyframes pulse {
		0% {
			opacity: 0.7;
			transform: scaleY(0.95);
		}
		100% {
			opacity: 1;
			transform: scaleY(1);
		}
	}
</style>
