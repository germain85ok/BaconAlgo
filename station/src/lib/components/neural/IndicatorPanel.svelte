<script lang="ts">
	import type { IndicatorValues } from '$lib/types';
	import GlassPanel from '$lib/components/ui/GlassPanel.svelte';
	
	export let indicators: IndicatorValues = {
		rsi: 65,
		macd: { value: 0.5, signal: 0.3, histogram: 0.2 },
		stochastic: { k: 70, d: 65 },
		ma50: 45000,
		ma200: 42000,
		ema21: 46000,
		volume_sma: 1000000
	};

	function getRSIColor(rsi: number): string {
		if (rsi > 70) return 'text-red-400';
		if (rsi < 30) return 'text-green-400';
		return 'text-yellow-400';
	}

	function getMAStatus(current: number, ma50: number, ma200: number): string {
		if (current > ma50 && ma50 > ma200) return 'Haussier';
		if (current < ma50 && ma50 < ma200) return 'Baissier';
		return 'Neutre';
	}
</script>

<GlassPanel variant="default" className="p-6">
	<h3 class="text-xl font-display font-bold text-bacon-orange mb-4">
		Indicateurs Techniques
	</h3>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
		<!-- Leading Indicators -->
		<div>
			<h4 class="text-sm font-semibold text-gray-400 mb-3 flex items-center gap-2">
				<span>🎯</span> Indicateurs Leading
			</h4>
			
			<div class="space-y-4">
				<!-- RSI -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm text-gray-400">RSI (14)</span>
						<span class="font-mono font-bold {getRSIColor(indicators.rsi)}">
							{indicators.rsi.toFixed(1)}
						</span>
					</div>
					<div class="w-full h-2 bg-white/10 rounded-full overflow-hidden">
						<div 
							class="h-full {getRSIColor(indicators.rsi)} bg-current transition-all"
							style="width: {indicators.rsi}%"
						></div>
					</div>
					<div class="flex justify-between text-xs text-gray-500 mt-1">
						<span>Survente (30)</span>
						<span>Surachat (70)</span>
					</div>
				</div>

				<!-- Stochastic -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm text-gray-400">Stochastique</span>
						<div class="text-xs">
							<span class="text-gray-400">K:</span>
							<span class="font-mono ml-1">{indicators.stochastic.k.toFixed(1)}</span>
							<span class="text-gray-400 ml-2">D:</span>
							<span class="font-mono ml-1">{indicators.stochastic.d.toFixed(1)}</span>
						</div>
					</div>
					<div class="w-full h-2 bg-white/10 rounded-full overflow-hidden">
						<div 
							class="h-full bg-purple-500 transition-all"
							style="width: {indicators.stochastic.k}%"
						></div>
					</div>
				</div>

				<!-- MACD -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm text-gray-400">MACD</span>
						<span class="font-mono text-xs {indicators.macd.histogram > 0 ? 'text-green-400' : 'text-red-400'}">
							{indicators.macd.histogram > 0 ? '↑' : '↓'} {Math.abs(indicators.macd.histogram).toFixed(3)}
						</span>
					</div>
					<div class="flex gap-2 text-xs">
						<div>
							<span class="text-gray-500">Signal:</span>
							<span class="font-mono ml-1">{indicators.macd.signal.toFixed(3)}</span>
						</div>
						<div>
							<span class="text-gray-500">Value:</span>
							<span class="font-mono ml-1">{indicators.macd.value.toFixed(3)}</span>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Lagging Indicators -->
		<div>
			<h4 class="text-sm font-semibold text-gray-400 mb-3 flex items-center gap-2">
				<span>📊</span> Indicateurs Lagging
			</h4>
			
			<div class="space-y-4">
				<!-- MA50 -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between">
						<span class="text-sm text-gray-400">MA 50</span>
						<span class="font-mono font-semibold text-blue-400">
							${indicators.ma50.toLocaleString()}
						</span>
					</div>
				</div>

				<!-- MA200 -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between">
						<span class="text-sm text-gray-400">MA 200</span>
						<span class="font-mono font-semibold text-blue-400">
							${indicators.ma200.toLocaleString()}
						</span>
					</div>
				</div>

				<!-- EMA21 -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between">
						<span class="text-sm text-gray-400">EMA 21</span>
						<span class="font-mono font-semibold text-purple-400">
							${indicators.ema21.toLocaleString()}
						</span>
					</div>
				</div>

				<!-- Volume -->
				<div class="indicator-box p-3 rounded-lg bg-white/5 border border-white/10">
					<div class="flex items-center justify-between">
						<span class="text-sm text-gray-400">Volume SMA</span>
						<span class="font-mono text-xs text-gray-300">
							{(indicators.volume_sma / 1000000).toFixed(2)}M
						</span>
					</div>
				</div>
			</div>
		</div>
	</div>
</GlassPanel>

<style>
	.indicator-box {
		transition: all 0.3s ease;
	}

	.indicator-box:hover {
		border-color: rgba(255, 107, 53, 0.3);
		background: rgba(255, 255, 255, 0.08);
	}
</style>
