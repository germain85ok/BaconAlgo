<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	const viewerCount = tweened(0, { duration: 1000, easing: cubicOut });

	let interval: ReturnType<typeof setInterval>;
	let baseCount = 1247;

	function updateViewerCount() {
		const variance = Math.floor(Math.random() * 20) - 5;
		baseCount = Math.max(10, baseCount + variance);
		viewerCount.set(baseCount);
	}

	onMount(() => {
		viewerCount.set(baseCount);
		interval = setInterval(updateViewerCount, 5000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});

	function formatCount(count: number): string {
		return Math.floor(count).toLocaleString();
	}
</script>

<div class="viewer-count">
	<div class="icon-wrapper">
		<svg
			class="eye-icon"
			width="32"
			height="32"
			viewBox="0 0 24 24"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<path
				d="M12 5C7 5 2.73 8.11 1 12.5C2.73 16.89 7 20 12 20C17 20 21.27 16.89 23 12.5C21.27 8.11 17 5 12 5Z"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
			<circle cx="12" cy="12.5" r="3.5" stroke="currentColor" stroke-width="2" />
		</svg>
		<div class="pulse-ring"></div>
	</div>

	<div class="count-display">
		<span class="count">{formatCount($viewerCount)}</span>
		<span class="label">Watching Now</span>
	</div>

	<div class="live-indicator">
		<span class="dot"></span>
		LIVE
	</div>
</div>

<style>
	.viewer-count {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		min-width: 200px;
	}

	.icon-wrapper {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.eye-icon {
		color: #f97316;
		filter: drop-shadow(0 0 10px rgba(249, 115, 22, 0.5));
		z-index: 1;
	}

	.pulse-ring {
		position: absolute;
		width: 48px;
		height: 48px;
		border: 2px solid #f97316;
		border-radius: 50%;
		animation: pulse-ring 2s ease-out infinite;
		opacity: 0;
	}

	@keyframes pulse-ring {
		0% {
			transform: scale(0.8);
			opacity: 1;
		}
		100% {
			transform: scale(1.5);
			opacity: 0;
		}
	}

	.count-display {
		text-align: center;
	}

	.count {
		display: block;
		font-size: 2rem;
		font-weight: 700;
		color: rgba(255, 255, 255, 0.95);
		line-height: 1;
		text-shadow: 0 0 20px rgba(249, 115, 22, 0.3);
	}

	.label {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.live-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid #ef4444;
		padding: 0.5rem 1rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 700;
		color: #ef4444;
		letter-spacing: 0.1em;
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #ef4444;
		animation: pulse-dot 2s ease-in-out infinite;
	}

	@keyframes pulse-dot {
		0%,
		100% {
			opacity: 1;
			box-shadow: 0 0 10px #ef4444;
		}
		50% {
			opacity: 0.5;
			box-shadow: 0 0 5px #ef4444;
		}
	}
</style>
