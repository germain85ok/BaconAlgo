<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	interface TimeZone {
		city: string;
		offset: number;
		marketHours: { open: number; close: number };
	}

	const timezones: TimeZone[] = [
		{ city: 'New York', offset: -5, marketHours: { open: 9.5, close: 16 } },
		{ city: 'London', offset: 0, marketHours: { open: 8, close: 16.5 } },
		{ city: 'Tokyo', offset: 9, marketHours: { open: 9, close: 15 } }
	];

	let currentTimes: { [key: string]: Date } = {};
	let interval: ReturnType<typeof setInterval>;

	function updateTimes() {
		const now = new Date();
		timezones.forEach((tz) => {
			const utcTime = now.getTime() + now.getTimezoneOffset() * 60000;
			const tzTime = new Date(utcTime + 3600000 * tz.offset);
			currentTimes[tz.city] = tzTime;
		});
		currentTimes = { ...currentTimes };
	}

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			hour12: false
		});
	}

	function isMarketOpen(city: string): boolean {
		const time = currentTimes[city];
		if (!time) return false;

		const tz = timezones.find((t) => t.city === city);
		if (!tz) return false;

		const day = time.getDay();
		if (day === 0 || day === 6) return false;

		const hours = time.getHours() + time.getMinutes() / 60;
		return hours >= tz.marketHours.open && hours < tz.marketHours.close;
	}

	onMount(() => {
		updateTimes();
		interval = setInterval(updateTimes, 1000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});
</script>

<div class="live-clock">
	<h3>Market Hours</h3>
	<div class="clocks-container">
		{#each timezones as tz (tz.city)}
			<div class="clock-card">
				<div class="city-name">{tz.city}</div>
				<div class="time-display">{currentTimes[tz.city] ? formatTime(currentTimes[tz.city]) : '--:--:--'}</div>
				<div class="market-status" class:open={isMarketOpen(tz.city)}>
					<span class="status-dot"></span>
					{isMarketOpen(tz.city) ? 'Market Open' : 'Market Closed'}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.live-clock {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 350px;
	}

	h3 {
		margin: 0 0 1rem 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.clocks-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.clock-card {
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
		padding: 1rem;
		transition: all 0.3s ease;
	}

	.clock-card:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: translateX(4px);
	}

	.city-name {
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 0.5rem;
	}

	.time-display {
		font-size: 1.75rem;
		font-weight: 700;
		color: rgba(255, 255, 255, 0.95);
		font-variant-numeric: tabular-nums;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
		font-family: 'Courier New', monospace;
	}

	.market-status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #6b7280;
		transition: all 0.3s ease;
	}

	.market-status.open {
		color: #10b981;
	}

	.market-status.open .status-dot {
		background: #10b981;
		box-shadow: 0 0 10px #10b981;
		animation: pulse-dot 2s ease-in-out infinite;
	}

	@keyframes pulse-dot {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.7;
			transform: scale(1.1);
		}
	}
</style>
