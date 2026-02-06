<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { apiClient } from '$lib/services/apiClient';
	import type { Signal } from '$lib/types';
	import GlassPanel from '$lib/components/ui/GlassPanel.svelte';
	
	export let symbol: string = 'BTCUSD';
	
	let signals: Signal[] = [];
	let latestSignal: Signal | null = null;
	let unsubscribe: (() => void) | null = null;

	onMount(() => {
		loadSignals();
		
		unsubscribe = apiClient.subscribe('SIGNAL', (data: Signal) => {
			if (data.symbol === symbol) {
				latestSignal = data;
				signals = [data, ...signals].slice(0, 10);
			}
		});
	});

	onDestroy(() => {
		if (unsubscribe) unsubscribe();
	});

	async function loadSignals() {
		const response = await apiClient.getSignals(10);
		if (response.success && response.data) {
			signals = response.data.filter(s => s.symbol === symbol);
			if (signals.length > 0) {
				latestSignal = signals[0];
			}
		}
	}

	function getConfidenceColor(confidence: number): string {
		if (confidence >= 80) return 'text-green-400';
		if (confidence >= 60) return 'text-yellow-400';
		return 'text-orange-400';
	}

	function getDirectionColor(direction: string): string {
		return direction === 'LONG' ? 'text-green-400' : 'text-red-400';
	}
</script>

<GlassPanel variant="bordered" className="p-6">
	<div class="mb-4">
		<h2 class="text-2xl font-display font-bold text-bacon-orange flex items-center gap-2">
			<span class="pulse-glow">⚡</span>
			Panneau Neural
		</h2>
		<p class="text-gray-400 text-sm mt-1">Signaux en temps réel - {symbol}</p>
	</div>

	{#if latestSignal}
		<div class="latest-signal mb-6 p-4 rounded-lg border-2 {latestSignal.direction === 'LONG' ? 'border-green-500/50 bg-green-500/5' : 'border-red-500/50 bg-red-500/5'}">
			<div class="flex items-center justify-between mb-3">
				<div class="flex items-center gap-3">
					<span class="text-2xl font-bold {getDirectionColor(latestSignal.direction)}">
						{latestSignal.direction}
					</span>
					<span class="text-sm text-gray-400">{latestSignal.timeframe}</span>
				</div>
				<div class="text-right">
					<div class="text-xs text-gray-400">Score Neural</div>
					<div class="text-xl font-bold text-bacon-orange">{latestSignal.neural_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="grid grid-cols-3 gap-4 text-sm">
				<div>
					<div class="text-gray-400">Entrée</div>
					<div class="font-mono font-semibold">${latestSignal.entry_price.toFixed(2)}</div>
				</div>
				<div>
					<div class="text-gray-400">Stop Loss</div>
					<div class="font-mono font-semibold text-red-400">${latestSignal.stop_loss.toFixed(2)}</div>
				</div>
				<div>
					<div class="text-gray-400">Take Profit</div>
					<div class="font-mono font-semibold text-green-400">${latestSignal.take_profit.toFixed(2)}</div>
				</div>
			</div>

			<div class="mt-3 flex items-center justify-between text-sm">
				<div>
					<span class="text-gray-400">Confiance:</span>
					<span class="ml-2 font-semibold {getConfidenceColor(latestSignal.confidence)}">
						{latestSignal.confidence.toFixed(1)}%
					</span>
				</div>
				<div>
					<span class="text-gray-400">R/R:</span>
					<span class="ml-2 font-semibold text-bacon-orange">
						1:{latestSignal.risk_reward.toFixed(2)}
					</span>
				</div>
			</div>
		</div>
	{:else}
		<div class="text-center py-8 text-gray-500">
			En attente de signaux...
		</div>
	{/if}

	<div class="space-y-2">
		<h3 class="text-sm font-semibold text-gray-400 mb-3">Historique récent</h3>
		{#each signals.slice(1, 6) as signal}
			<div class="signal-item p-3 rounded-lg bg-white/5 border border-white/10 hover:border-bacon-orange/30 transition-all">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-3">
						<span class="text-sm font-semibold {getDirectionColor(signal.direction)}">
							{signal.direction}
						</span>
						<span class="text-xs text-gray-500">{signal.timeframe}</span>
					</div>
					<div class="flex items-center gap-4 text-xs">
						<span class="text-gray-400">
							{new Date(signal.timestamp).toLocaleTimeString('fr-CA')}
						</span>
						<span class={getConfidenceColor(signal.confidence)}>
							{signal.confidence.toFixed(0)}%
						</span>
					</div>
				</div>
			</div>
		{/each}
	</div>
</GlassPanel>

<style>
	.pulse-glow {
		animation: pulseGlow 2s ease-in-out infinite;
	}

	@keyframes pulseGlow {
		0%, 100% {
			filter: drop-shadow(0 0 5px rgba(255, 107, 53, 0.5));
		}
		50% {
			filter: drop-shadow(0 0 15px rgba(255, 107, 53, 0.8));
		}
	}
</style>
