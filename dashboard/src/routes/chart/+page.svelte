<script lang="ts">
  import BaconChart from '$lib/components/Chart/BaconChart.svelte';
  import { onMount } from 'svelte';
  
  let selectedSymbol = 'BTCUSDT';
  let selectedTimeframe = '15';
  let signals: any[] = [];
  let loading = false;
  
  // SMC toggle states
  let showFVG = true;
  let showOB = true;
  let showBOS = true;
  let showLiquidity = true;
  let showVWAP = true;
  let showPrevDayLevels = true;
  let showFibonacci = true;
  let showSignals = true;
  
  onMount(async () => {
    await loadSignals();
  });
  
  async function loadSignals() {
    loading = true;
    try {
      const response = await fetch('http://localhost:8000/api/signals');
      if (response.ok) {
        const data = await response.json();
        signals = data.signals || [];
      }
    } catch (error) {
      console.error('Failed to load signals:', error);
      // Use mock signals
      signals = [
        {
          id: 'BTC-15m-001',
          symbol: 'BTCUSDT',
          timeframe: '15m',
          direction: 'LONG',
          score: 88,
          entry: 50250.00,
          stopLoss: 49800.00,
          takeProfit: 51600.00,
          riskRewardRatio: 3.0,
          confidence: 0.88,
          reasons: ['Bullish FVG', 'Order Block Support', 'BOS Confirmed'],
          createdAt: new Date().toISOString(),
        },
        {
          id: 'ETH-15m-002',
          symbol: 'ETHUSDT',
          timeframe: '15m',
          direction: 'LONG',
          score: 82,
          entry: 2850.00,
          stopLoss: 2820.00,
          takeProfit: 2940.00,
          riskRewardRatio: 3.0,
          confidence: 0.82,
          reasons: ['VWAP Support', 'Bullish Structure'],
          createdAt: new Date().toISOString(),
        },
      ];
    } finally {
      loading = false;
    }
  }
  
  function handleSymbolChange(signal: any) {
    selectedSymbol = signal.symbol;
    selectedTimeframe = signal.timeframe;
  }
</script>

<svelte:head>
  <title>Chart - BaconAlgo</title>
</svelte:head>

<div class="chart-page">
  <header class="page-header">
    <div class="header-left">
      <h1>🥓 BaconAlgo Chart</h1>
      <p class="subtitle">Advanced SMC Trading Visualization</p>
    </div>
    <div class="header-right">
      <div class="stats">
        <div class="stat">
          <span class="stat-label">Active Signals</span>
          <span class="stat-value">{signals.length}</span>
        </div>
        <div class="stat">
          <span class="stat-label">Symbol</span>
          <span class="stat-value">{selectedSymbol}</span>
        </div>
      </div>
    </div>
  </header>
  
  <div class="content">
    <div class="main-panel">
      <BaconChart
        symbol={selectedSymbol}
        timeframe={selectedTimeframe}
        {showFVG}
        {showOB}
        {showBOS}
        {showLiquidity}
        {showVWAP}
        {showPrevDayLevels}
        {showFibonacci}
        {showSignals}
      />
    </div>
    
    <aside class="side-panel">
      <div class="panel-section">
        <h3>🎛️ Chart Controls</h3>
        <div class="toggle-list">
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showFVG} />
            <span>Fair Value Gaps (FVG)</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showOB} />
            <span>Order Blocks (OB)</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showBOS} />
            <span>Break of Structure (BOS)</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showLiquidity} />
            <span>Liquidity Zones</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showVWAP} />
            <span>VWAP</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showPrevDayLevels} />
            <span>Previous Day Levels</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showFibonacci} />
            <span>Fibonacci</span>
          </label>
          <label class="toggle-item">
            <input type="checkbox" bind:checked={showSignals} />
            <span>Entry Signals</span>
          </label>
        </div>
      </div>
      
      <div class="panel-section">
        <h3>🎯 Active Signals</h3>
        {#if loading}
          <div class="loading">Loading signals...</div>
        {:else if signals.length === 0}
          <div class="empty-state">
            <p>No active signals</p>
          </div>
        {:else}
          <div class="signals-list">
            {#each signals as signal}
              <button
                class="signal-card"
                class:active={selectedSymbol === signal.symbol}
                on:click={() => handleSymbolChange(signal)}
              >
                <div class="signal-header">
                  <span class="signal-symbol">{signal.symbol}</span>
                  <span class="signal-score" class:high={signal.score >= 85}>
                    {signal.score}
                  </span>
                </div>
                <div class="signal-direction" class:long={signal.direction === 'LONG'} class:short={signal.direction === 'SHORT'}>
                  {signal.direction}
                </div>
                <div class="signal-details">
                  <div class="detail">
                    <span class="label">Entry:</span>
                    <span class="value">${signal.entry.toLocaleString()}</span>
                  </div>
                  <div class="detail">
                    <span class="label">SL:</span>
                    <span class="value">${signal.stopLoss.toLocaleString()}</span>
                  </div>
                  <div class="detail">
                    <span class="label">TP:</span>
                    <span class="value">${signal.takeProfit.toLocaleString()}</span>
                  </div>
                  <div class="detail">
                    <span class="label">R:R:</span>
                    <span class="value">{signal.riskRewardRatio}:1</span>
                  </div>
                </div>
                <div class="signal-reasons">
                  {#each signal.reasons.slice(0, 2) as reason}
                    <span class="reason-tag">{reason}</span>
                  {/each}
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </aside>
  </div>
</div>

<style>
  .chart-page {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #0a0a0f;
    color: #d1d4dc;
  }
  
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 30px;
    background: #12121a;
    border-bottom: 1px solid #1a1a2e;
  }
  
  .header-left h1 {
    margin: 0;
    font-size: 28px;
    font-weight: 700;
    color: #fff;
  }
  
  .subtitle {
    margin: 5px 0 0 0;
    font-size: 14px;
    color: #888;
  }
  
  .header-right .stats {
    display: flex;
    gap: 30px;
  }
  
  .stat {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }
  
  .stat-label {
    font-size: 11px;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .stat-value {
    font-size: 20px;
    font-weight: 700;
    color: #00ff88;
  }
  
  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  
  .main-panel {
    flex: 1;
    padding: 20px;
    overflow: auto;
  }
  
  .side-panel {
    width: 350px;
    background: #12121a;
    border-left: 1px solid #1a1a2e;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0;
  }
  
  .panel-section {
    padding: 20px;
    border-bottom: 1px solid #1a1a2e;
  }
  
  .panel-section h3 {
    margin: 0 0 15px 0;
    font-size: 16px;
    font-weight: 600;
    color: #fff;
  }
  
  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  
  .toggle-item {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    padding: 8px 10px;
    border-radius: 6px;
    transition: background 0.2s;
  }
  
  .toggle-item:hover {
    background: rgba(255, 255, 255, 0.05);
  }
  
  .toggle-item input[type="checkbox"] {
    cursor: pointer;
  }
  
  .toggle-item span {
    font-size: 14px;
    color: #d1d4dc;
  }
  
  .signals-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  
  .signal-card {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 15px;
    background: #1a1a2e;
    border: 1px solid #2a2a3e;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
    width: 100%;
  }
  
  .signal-card:hover {
    background: #222232;
    border-color: #00ff88;
  }
  
  .signal-card.active {
    background: #1a2a1f;
    border-color: #00ff88;
  }
  
  .signal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .signal-symbol {
    font-size: 16px;
    font-weight: 700;
    color: #fff;
  }
  
  .signal-score {
    font-size: 14px;
    font-weight: 700;
    color: #ffcc00;
    background: rgba(255, 204, 0, 0.15);
    padding: 4px 8px;
    border-radius: 4px;
  }
  
  .signal-score.high {
    color: #00ff88;
    background: rgba(0, 255, 136, 0.15);
  }
  
  .signal-direction {
    font-size: 13px;
    font-weight: 700;
    padding: 6px 12px;
    border-radius: 4px;
    display: inline-block;
    width: fit-content;
  }
  
  .signal-direction.long {
    background: rgba(0, 255, 136, 0.15);
    color: #00ff88;
  }
  
  .signal-direction.short {
    background: rgba(255, 68, 102, 0.15);
    color: #ff4466;
  }
  
  .signal-details {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }
  
  .detail {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
  }
  
  .detail .label {
    color: #888;
  }
  
  .detail .value {
    color: #fff;
    font-weight: 600;
  }
  
  .signal-reasons {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  
  .reason-tag {
    font-size: 10px;
    padding: 4px 8px;
    background: rgba(68, 136, 255, 0.15);
    color: #4488ff;
    border-radius: 4px;
    font-weight: 600;
  }
  
  .loading,
  .empty-state {
    text-align: center;
    padding: 20px;
    color: #888;
    font-size: 14px;
  }
</style>
