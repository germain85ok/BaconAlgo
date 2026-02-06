<script lang="ts">
	export let confidence: number = 75;
	export let neuralScore: number = 82;
	export let showLabel: boolean = true;

	function getConfidenceLevel(score: number): string {
		if (score >= 80) return 'Très Élevée';
		if (score >= 60) return 'Élevée';
		if (score >= 40) return 'Moyenne';
		return 'Faible';
	}

	function getConfidenceColor(score: number): string {
		if (score >= 80) return 'from-green-500 to-emerald-600';
		if (score >= 60) return 'from-yellow-500 to-orange-500';
		if (score >= 40) return 'from-orange-500 to-red-500';
		return 'from-red-500 to-red-700';
	}
</script>

<div class="confidence-score">
	{#if showLabel}
		<div class="flex items-center justify-between mb-2">
			<span class="text-sm text-gray-400">Score de Confiance</span>
			<span class="text-sm font-semibold text-bacon-orange">
				{getConfidenceLevel(confidence)}
			</span>
		</div>
	{/if}

	<div class="confidence-bars space-y-2">
		<!-- Confidence Bar -->
		<div>
			<div class="flex items-center justify-between text-xs mb-1">
				<span class="text-gray-400">Confiance</span>
				<span class="font-mono font-bold">{confidence.toFixed(1)}%</span>
			</div>
			<div class="w-full h-3 bg-white/10 rounded-full overflow-hidden relative">
				<div 
					class="h-full bg-gradient-to-r {getConfidenceColor(confidence)} transition-all duration-500 confidence-fill"
					style="width: {confidence}%"
				></div>
			</div>
		</div>

		<!-- Neural Score Bar -->
		<div>
			<div class="flex items-center justify-between text-xs mb-1">
				<span class="text-gray-400">Score Neural AI</span>
				<span class="font-mono font-bold text-bacon-orange">{neuralScore.toFixed(1)}%</span>
			</div>
			<div class="w-full h-3 bg-white/10 rounded-full overflow-hidden relative">
				<div 
					class="h-full bg-gradient-to-r from-bacon-orange to-orange-600 transition-all duration-500 neural-fill"
					style="width: {neuralScore}%"
				></div>
			</div>
		</div>
	</div>

	<!-- Visual Indicator -->
	<div class="mt-4 flex items-center justify-center">
		<div class="confidence-circle {confidence >= 80 ? 'high' : confidence >= 60 ? 'medium' : 'low'}">
			<div class="circle-content">
				<div class="text-3xl font-bold font-display">{Math.round(confidence)}</div>
				<div class="text-xs text-gray-400">SCORE</div>
			</div>
		</div>
	</div>
</div>

<style>
	.confidence-fill,
	.neural-fill {
		box-shadow: 0 0 10px currentColor;
	}

	.confidence-circle {
		width: 120px;
		height: 120px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		border: 3px solid;
	}

	.confidence-circle.high {
		border-color: #10b981;
		box-shadow: 0 0 20px rgba(16, 185, 129, 0.5);
		background: radial-gradient(circle, rgba(16, 185, 129, 0.1), transparent);
	}

	.confidence-circle.medium {
		border-color: #f59e0b;
		box-shadow: 0 0 20px rgba(245, 158, 11, 0.5);
		background: radial-gradient(circle, rgba(245, 158, 11, 0.1), transparent);
	}

	.confidence-circle.low {
		border-color: #ef4444;
		box-shadow: 0 0 20px rgba(239, 68, 68, 0.5);
		background: radial-gradient(circle, rgba(239, 68, 68, 0.1), transparent);
	}

	.circle-content {
		text-align: center;
	}
</style>
