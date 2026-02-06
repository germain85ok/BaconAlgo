<script lang="ts">
	import type { Timeframe } from '$lib/types';
	import GlassPanel from '$lib/components/ui/GlassPanel.svelte';
	
	export let symbol: string = 'BTCUSD';
	export let activeTimeframe: Timeframe = '1h';
	
	const timeframes: Timeframe[] = ['1m', '5m', '15m', '1h', '4h', '1D', '1W'];
	
	let timeframeData: Record<Timeframe, { trend: 'UP' | 'DOWN' | 'NEUTRAL'; strength: number }> = {
		'1m': { trend: 'NEUTRAL', strength: 50 },
		'5m': { trend: 'UP', strength: 65 },
		'15m': { trend: 'UP', strength: 72 },
		'1h': { trend: 'UP', strength: 80 },
		'4h': { trend: 'UP', strength: 75 },
		'1D': { trend: 'UP', strength: 68 },
		'1W': { trend: 'UP', strength: 85 }
	};

	function getTrendIcon(trend: string): string {
		if (trend === 'UP') return '📈';
		if (trend === 'DOWN') return '📉';
		return '➡️';
	}

	function getTrendColor(trend: string): string {
		if (trend === 'UP') return 'text-green-400';
		if (trend === 'DOWN') return 'text-red-400';
		return 'text-gray-400';
	}

	function getStrengthColor(strength: number): string {
		if (strength >= 75) return 'bg-green-500';
		if (strength >= 50) return 'bg-yellow-500';
		return 'bg-red-500';
	}
</script>

<GlassPanel variant="default" className="p-6">
	<h3 class="text-xl font-display font-bold text-bacon-orange mb-4">
		Multi-Timeframe Analysis
	</h3>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
		{#each timeframes as tf}
			<button
				class="timeframe-card p-4 rounded-lg border transition-all {activeTimeframe === tf ? 'border-bacon-orange bg-bacon-orange/10' : 'border-white/10 bg-white/5'}"
				on:click={() => activeTimeframe = tf}
			>
				<div class="flex items-center justify-between mb-2">
					<span class="font-mono font-bold text-lg">{tf}</span>
					<span class="text-xl">{getTrendIcon(timeframeData[tf].trend)}</span>
				</div>
				
				<div class="mb-2">
					<span class="text-sm {getTrendColor(timeframeData[tf].trend)} font-semibold">
						{timeframeData[tf].trend}
					</span>
				</div>

				<div class="strength-bar w-full h-2 bg-white/10 rounded-full overflow-hidden">
					<div 
						class="h-full {getStrengthColor(timeframeData[tf].strength)} transition-all"
						style="width: {timeframeData[tf].strength}%"
					></div>
				</div>
				<div class="text-xs text-gray-400 mt-1 text-right">
					{timeframeData[tf].strength}% force
				</div>
			</button>
		{/each}
	</div>

	<div class="mt-6 p-4 rounded-lg bg-bacon-orange/10 border border-bacon-orange/30">
		<div class="flex items-center gap-2 mb-2">
			<span class="text-bacon-orange font-bold">⚡ Consensus Multi-TF</span>
		</div>
		<p class="text-sm text-gray-300">
			5 sur 7 timeframes montrent une tendance haussière avec une force moyenne de 72%
		</p>
	</div>
</GlassPanel>

<style>
	.timeframe-card {
		cursor: pointer;
	}

	.timeframe-card:hover {
		border-color: rgba(255, 107, 53, 0.5);
		box-shadow: 0 0 15px rgba(255, 107, 53, 0.2);
	}
</style>
