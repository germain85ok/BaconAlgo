<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { marketStatus } from '$lib/stores/market';

	let currentTime = new Date();
	let timeUntilOpen = '';
	let isOpen = false;
	let interval: ReturnType<typeof setInterval>;

	function updateTime() {
		currentTime = new Date();
		const status = $marketStatus;
		isOpen = status.isOpen;

		if (!isOpen && status.nextOpen) {
			const diff = status.nextOpen.getTime() - currentTime.getTime();
			const hours = Math.floor(diff / (1000 * 60 * 60));
			const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
			const seconds = Math.floor((diff % (1000 * 60)) / 1000);

			timeUntilOpen = `${hours}h ${minutes}m ${seconds}s`;
		}
	}

	onMount(() => {
		updateTime();
		interval = setInterval(updateTime, 1000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			hour12: true
		});
	}
</script>

<div class="market-countdown glass-card">
	<div class="current-time">
		<span class="time-label">Current Time</span>
		<span class="time-value">{formatTime(currentTime)}</span>
	</div>

	<div class="status-divider"></div>

	<div class="market-status">
		{#if isOpen}
			<div class="status-indicator open">
				<span class="pulse"></span>
				<span class="status-text">Market Open</span>
			</div>
		{:else}
			<div class="status-indicator closed">
				<span class="status-text">Market Closed</span>
			</div>
			<div class="countdown">
				<span class="countdown-label">Opens in:</span>
				<span class="countdown-value">{timeUntilOpen}</span>
			</div>
		{/if}
	</div>
</div>

<style>
	.market-countdown {
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 16px 24px;
		min-width: 400px;
	}

	.current-time {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.time-label {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.6);
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.time-value {
		font-size: 24px;
		font-weight: 700;
		color: #ffffff;
		font-variant-numeric: tabular-nums;
	}

	.status-divider {
		width: 1px;
		height: 40px;
		background: rgba(255, 255, 255, 0.2);
	}

	.market-status {
		display: flex;
		flex-direction: column;
		gap: 8px;
		flex: 1;
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.status-indicator.open .status-text {
		color: #2ecc71;
		font-weight: 600;
		font-size: 16px;
	}

	.status-indicator.closed .status-text {
		color: #e74c3c;
		font-weight: 600;
		font-size: 16px;
	}

	.pulse {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #2ecc71;
		box-shadow: 0 0 0 0 rgba(46, 204, 113, 0.7);
		animation: pulse 2s infinite;
	}

	@keyframes pulse {
		0% {
			box-shadow: 0 0 0 0 rgba(46, 204, 113, 0.7);
		}
		70% {
			box-shadow: 0 0 0 10px rgba(46, 204, 113, 0);
		}
		100% {
			box-shadow: 0 0 0 0 rgba(46, 204, 113, 0);
		}
	}

	.countdown {
		display: flex;
		align-items: baseline;
		gap: 8px;
	}

	.countdown-label {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.6);
	}

	.countdown-value {
		font-size: 18px;
		font-weight: 700;
		color: #ff6b35;
		font-variant-numeric: tabular-nums;
	}
</style>
