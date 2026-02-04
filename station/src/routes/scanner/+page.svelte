<script>
  import { onMount } from 'svelte';
  import GlassCard from '$lib/components/GlassCard.svelte';
  import SignalCard from '$lib/components/SignalCard.svelte';
  
  let signals = [];
  let filterType = 'all';
  let minScore = 60;
  let loading = false;
  
  async function fetchSignals() {
    loading = true;
    try {
      const params = new URLSearchParams();
      if (filterType !== 'all') params.append('type', filterType);
      params.append('min_score', minScore);
      
      const response = await fetch(`http://localhost:3000/api/scan/all?${params}`);
      signals = await response.json();
    } catch (error) {
      console.error('Failed to fetch signals:', error);
      // Use mock data for demo
      signals = [
        {
          symbol: 'BTCUSDT',
          timeframe: 'M15',
          score: 85,
          signal_type: 'day',
          direction: 'bullish',
          entry_price: 45000,
          stop_loss: 44000,
          target_price: 47000,
          tags: ['fvg', 'wave3']
        },
        {
          symbol: 'ETHUSDT',
          timeframe: 'M15',
          score: 72,
          signal_type: 'day',
          direction: 'bullish',
          entry_price: 3000,
          stop_loss: 2900,
          target_price: 3200,
          tags: ['orb_breakout']
        }
      ];
    }
    loading = false;
  }
  
  onMount(() => {
    fetchSignals();
  });
  
  function handleFilterChange() {
    fetchSignals();
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-4xl font-display font-bold text-gradient">Scanner</h1>
    <button on:click={fetchSignals} class="btn-primary" disabled={loading}>
      {loading ? '⏳ Scanning...' : '🔄 Refresh'}
    </button>
  </div>
  
  <!-- Filters -->
  <GlassCard className="p-6">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div>
        <label class="block text-sm text-gray-400 mb-2">Signal Type</label>
        <select bind:value={filterType} on:change={handleFilterChange} class="input w-full">
          <option value="all">All</option>
          <option value="scalp">Scalp (M1-M5)</option>
          <option value="day">Day Trading (M15-H1)</option>
          <option value="swing">Swing (H4-D1)</option>
          <option value="long">Long Term (W1+)</option>
        </select>
      </div>
      
      <div>
        <label class="block text-sm text-gray-400 mb-2">Min Score: {minScore}%</label>
        <input 
          type="range" 
          bind:value={minScore} 
          on:change={handleFilterChange}
          min="0" 
          max="100" 
          class="w-full"
        />
      </div>
      
      <div>
        <label class="block text-sm text-gray-400 mb-2">Results</label>
        <div class="number text-2xl">{signals.length}</div>
      </div>
    </div>
  </GlassCard>
  
  <!-- Signals Grid -->
  {#if loading}
    <div class="text-center py-12">
      <div class="animate-spin text-6xl">⏳</div>
      <p class="mt-4 text-gray-400">Scanning markets...</p>
    </div>
  {:else if signals.length === 0}
    <GlassCard className="p-12 text-center">
      <div class="text-6xl mb-4">📊</div>
      <h3 class="text-xl font-display mb-2">No signals found</h3>
      <p class="text-gray-400">Try adjusting your filters</p>
    </GlassCard>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each signals as signal}
        <SignalCard {signal} />
      {/each}
    </div>
  {/if}
</div>
