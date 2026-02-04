<script lang="ts">
	import { vixValue } from '$lib/stores/market';
	import { onMount } from 'svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	const displayValue = tweened(0, { duration: 800, easing: cubicOut });
	let prevValue = 0;
	let trend: 'up' | 'down' | 'neutral' = 'neutral';

	$: {
		if ($vixValue !== prevValue) {
			trend = $vixValue > prevValue ? 'up' : $vixValue < prevValue ? 'down' : 'neutral';
			prevValue = $vixValue;
			displayValue.set($vixValue);
		}
	}

	$: level =
		$displayValue < 15
			? { color: '#10b981', label: 'Low', bg: 'rgba(16, 185, 129, 0.2)' }
			: $displayValue < 25
				? { color: '#eab308', label: 'Normal', bg: 'rgba(234, 179, 8, 0.2)' }
				: $displayValue < 35
					? { color: '#f97316', label: 'Elevated', bg: 'rgba(249, 115, 22, 0.2)' }
					: { color: '#ef4444', label: 'High', bg: 'rgba(239, 68, 68, 0.2)' };

	$: percentage = Math.min(($displayValue / 50) * 100, 100);

	onMount(() => {
		displayValue.set($vixValue);
	});
</script>

<div class="vix-meter">
	<div class="header">
		<h3>VIX Index</h3>
		<span class="trend trend-{trend}">
			{#if trend === 'up'}
				<svg
					width="16"
					height="16"
					viewBox="0 0 16 16"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path d="M8 3L13 11H3L8 3Z" fill="currentColor" />
				</svg>
			{:else if trend === 'down'}
				<svg
					width="16"
					height="16"
					viewBox="0 0 16 16"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path d="M8 13L13 5H3L8 13Z" fill="currentColor" />
				</svg>
			{:else}
				<svg
					width="16"
					height="16"
					viewBox="0 0 16 16"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path d="M3 8H13" stroke="currentColor" stroke-width="2" />
				</svg>
			{/if}
		</span>
	</div>

	<div class="value-display" style="color: {level.color}">
		<span class="value">{$displayValue.toFixed(2)}</span>
		<span class="label">{level.label}</span>
	</div>

	<div class="meter-container">
		<div class="meter-bg">
			<div class="meter-fill" style="width: {percentage}%; background: {level.color};" />
		</div>
		<div class="meter-markers">
			<span style="left: 30%">15</span>
			<span style="left: 50%">25</span>
			<span style="left: 70%">35</span>
		</div>
	</div>
</div>

<style>
	.vix-meter {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 280px;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
	}

	.trend {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.1);
	}

	.trend-up {
		color: #ef4444;
	}

	.trend-down {
		color: #10b981;
	}

	.trend-neutral {
		color: rgba(255, 255, 255, 0.6);
	}

	.value-display {
		text-align: center;
		margin-bottom: 1.5rem;
	}

	.value {
		display: block;
		font-size: 2.5rem;
		font-weight: 700;
		line-height: 1;
		text-shadow: 0 0 20px currentColor;
	}

	.label {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		opacity: 0.8;
	}

	.meter-container {
		position: relative;
	}

	.meter-bg {
		height: 12px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		overflow: hidden;
		position: relative;
	}

	.meter-fill {
		height: 100%;
		border-radius: 6px;
		transition: width 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94);
		box-shadow: 0 0 10px currentColor;
		position: relative;
		overflow: hidden;
	}

	.meter-fill::after {
		content: '';
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
		animation: shimmer 2s infinite;
	}

	@keyframes shimmer {
		0% {
			left: -100%;
		}
		100% {
			left: 100%;
		}
	}

	.meter-markers {
		display: flex;
		justify-content: space-between;
		margin-top: 0.5rem;
		position: relative;
	}

	.meter-markers span {
		position: absolute;
		transform: translateX(-50%);
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}
</style>
