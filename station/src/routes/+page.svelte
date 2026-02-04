<script>
  import GlassCard from '$lib/components/GlassCard.svelte';
  import SignalCard from '$lib/components/SignalCard.svelte';
  
  const mockSignals = [
    {
      symbol: 'BTCUSDT',
      timeframe: 'M15',
      score: 85,
      direction: 'bullish',
      entry_price: 45000,
      stop_loss: 44000,
      target_price: 47000,
      tags: ['golden_pocket', 'fvg', 'wave3']
    },
    {
      symbol: 'ETHUSDT',
      timeframe: 'H1',
      score: 72,
      direction: 'bullish',
      entry_price: 3000,
      stop_loss: 2900,
      target_price: 3200,
      tags: ['order_block', 'orb_breakout']
    }
  ];
</script>

<div class="space-y-8">
  <!-- Hero Section -->
  <div class="text-center py-12">
    <h1 class="text-6xl font-display font-bold text-gradient mb-4">
      BaconAlgo SAAS 2030
    </h1>
    <p class="text-xl text-gray-300 mb-8">
      Ultimate Trading Platform with Smart Money Analysis
    </p>
    <div class="flex justify-center gap-4">
      <a href="/scanner" class="btn-primary">Start Scanning</a>
      <a href="/pricing" class="btn-secondary">View Pricing</a>
    </div>
  </div>
  
  <!-- Stats Grid -->
  <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
    <GlassCard hover={true} className="p-6 text-center">
      <div class="text-4xl font-display font-bold text-gradient mb-2">1000+</div>
      <div class="text-sm text-gray-400">Symbols Scanned/sec</div>
    </GlassCard>
    
    <GlassCard hover={true} className="p-6 text-center">
      <div class="text-4xl font-display font-bold text-success mb-2">85%</div>
      <div class="text-sm text-gray-400">Avg Confluence Score</div>
    </GlassCard>
    
    <GlassCard hover={true} className="p-6 text-center">
      <div class="text-4xl font-display font-bold text-primary mb-2">15+</div>
      <div class="text-sm text-gray-400">Smart Money Indicators</div>
    </GlassCard>
    
    <GlassCard hover={true} className="p-6 text-center">
      <div class="text-4xl font-display font-bold text-accent mb-2">24/7</div>
      <div class="text-sm text-gray-400">Live Market Analysis</div>
    </GlassCard>
  </div>
  
  <!-- Top Signals -->
  <div>
    <h2 class="text-3xl font-display font-bold mb-6">🔥 Top Signals</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      {#each mockSignals as signal}
        <SignalCard {signal} />
      {/each}
    </div>
  </div>
  
  <!-- Features -->
  <div>
    <h2 class="text-3xl font-display font-bold mb-6 text-center">Features</h2>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <GlassCard hover={true} className="p-6">
        <div class="text-4xl mb-4">🦀</div>
        <h3 class="text-xl font-display font-bold mb-2">Ultra-Fast Rust Engine</h3>
        <p class="text-gray-400">Scan 1000+ symbols per second with advanced indicators</p>
      </GlassCard>
      
      <GlassCard hover={true} className="p-6">
        <div class="text-4xl mb-4">🎨</div>
        <h3 class="text-xl font-display font-bold mb-2">OPUS 2030 Design</h3>
        <p class="text-gray-400">Beautiful glassmorphism UI with smooth animations</p>
      </GlassCard>
      
      <GlassCard hover={true} className="p-6">
        <div class="text-4xl mb-4">🤖</div>
        <h3 class="text-xl font-display font-bold mb-2">AI Predictions</h3>
        <p class="text-gray-400">ML/AI powered price predictions and pattern detection</p>
      </GlassCard>
    </div>
  </div>
</div>
