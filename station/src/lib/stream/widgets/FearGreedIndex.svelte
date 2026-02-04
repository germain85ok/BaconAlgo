<script lang="ts">
	import { fearGreedIndex } from '$lib/stores/market';

	$: value = $fearGreedIndex;
	$: rotation = (value / 100) * 180 - 90; // -90 to 90 degrees
	$: color = getColor(value);
	$: sentiment = getSentiment(value);

	function getColor(val: number): string {
		if (val <= 25) return '#e74c3c'; // Extreme Fear - Red
		if (val <= 45) return '#f39c12'; // Fear - Orange
		if (val <= 55) return '#f1c40f'; // Neutral - Yellow
		if (val <= 75) return '#2ecc71'; // Greed - Green
		return '#27ae60'; // Extreme Greed - Dark Green
	}

	function getSentiment(val: number): string {
		if (val <= 25) return 'Extreme Fear';
		if (val <= 45) return 'Fear';
		if (val <= 55) return 'Neutral';
		if (val <= 75) return 'Greed';
		return 'Extreme Greed';
	}
</script>

<div class="fear-greed-index glass-card">
	<div class="header">
		<span class="title">Fear & Greed Index</span>
	</div>

	<div class="gauge-container">
		<svg class="gauge" viewBox="0 0 200 120" width="200" height="120">
			<!-- Background arc -->
			<path
				d="M 20 100 A 80 80 0 0 1 180 100"
				fill="none"
				stroke="rgba(255, 255, 255, 0.1)"
				stroke-width="12"
				stroke-linecap="round"
			/>

			<!-- Gradient arc -->
			<defs>
				<linearGradient id="gaugeGradient" x1="0%" y1="0%" x2="100%" y2="0%">
					<stop offset="0%" style="stop-color:#e74c3c;stop-opacity:1" />
					<stop offset="25%" style="stop-color:#f39c12;stop-opacity:1" />
					<stop offset="50%" style="stop-color:#f1c40f;stop-opacity:1" />
					<stop offset="75%" style="stop-color:#2ecc71;stop-opacity:1" />
					<stop offset="100%" style="stop-color:#27ae60;stop-opacity:1" />
				</linearGradient>
			</defs>

			<path
				d="M 20 100 A 80 80 0 0 1 180 100"
				fill="none"
				stroke="url(#gaugeGradient)"
				stroke-width="12"
				stroke-linecap="round"
				opacity="0.3"
			/>

			<!-- Needle -->
			<line
				x1="100"
				y1="100"
				x2="100"
				y2="30"
				stroke={color}
				stroke-width="3"
				stroke-linecap="round"
				transform="rotate({rotation} 100 100)"
				style="transition: transform 0.5s ease;"
			/>

			<!-- Center dot -->
			<circle cx="100" cy="100" r="6" fill={color} />
		</svg>

		<div class="value-display">
			<span class="value" style="color: {color}">{value}</span>
			<span class="sentiment">{sentiment}</span>
		</div>
	</div>
</div>

<style>
	.fear-greed-index {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 20px 24px;
		min-width: 240px;
	}

	.header {
		width: 100%;
		margin-bottom: 12px;
	}

	.title {
		font-size: 14px;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.8);
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.gauge-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}

	.gauge {
		filter: drop-shadow(0 0 10px rgba(255, 255, 255, 0.1));
	}

	.value-display {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		margin-top: -20px;
	}

	.value {
		font-size: 32px;
		font-weight: 700;
		transition: color 0.5s ease;
	}

	.sentiment {
		font-size: 12px;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.7);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
</style>
