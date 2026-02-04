<script>
  export let signal = {
    symbol: 'BTCUSDT',
    timeframe: 'M15',
    score: 85,
    direction: 'bullish',
    entry_price: 45000,
    stop_loss: 44000,
    target_price: 47000,
    tags: ['golden_pocket', 'fvg', 'wave3']
  };
  
  $: riskReward = ((signal.target_price - signal.entry_price) / (signal.entry_price - signal.stop_loss)).toFixed(2);
</script>

<div class="glass-card-hover p-6">
  <!-- Header -->
  <div class="flex items-start justify-between mb-4">
    <div>
      <h3 class="text-2xl font-display font-bold">{signal.symbol}</h3>
      <p class="text-sm text-gray-400">{signal.timeframe}</p>
    </div>
    <span class="badge {signal.direction === 'bullish' ? 'badge-bullish' : 'badge-bearish'}">
      {signal.direction.toUpperCase()}
    </span>
  </div>
  
  <!-- Score -->
  <div class="mb-4">
    <div class="flex justify-between items-center mb-2">
      <span class="text-sm text-gray-400">Confluence Score</span>
      <span class="number text-lg">{signal.score}%</span>
    </div>
    <div class="score-bar">
      <div class="score-bar-fill" style="width: {signal.score}%"></div>
    </div>
  </div>
  
  <!-- Prices -->
  <div class="grid grid-cols-2 gap-4 mb-4">
    <div>
      <p class="text-xs text-gray-400 mb-1">Entry</p>
      <p class="number text-xl">${signal.entry_price.toLocaleString()}</p>
    </div>
    <div>
      <p class="text-xs text-gray-400 mb-1">Target</p>
      <p class="number-bullish text-xl">${signal.target_price.toLocaleString()}</p>
    </div>
    <div>
      <p class="text-xs text-gray-400 mb-1">Stop Loss</p>
      <p class="number-bearish text-xl">${signal.stop_loss.toLocaleString()}</p>
    </div>
    <div>
      <p class="text-xs text-gray-400 mb-1">R:R</p>
      <p class="number text-xl">1:{riskReward}</p>
    </div>
  </div>
  
  <!-- Tags -->
  <div class="flex flex-wrap gap-2">
    {#each signal.tags as tag}
      <span class="badge badge-neutral text-xs">{tag}</span>
    {/each}
  </div>
</div>
