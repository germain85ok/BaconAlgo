<script lang="ts">
  import { onMount } from "svelte";
  import { UltimateScorer, createMockScores, type UltimateSignal } from "$lib/scoring/ultimateScore";

  let watchlist = ["AAPL", "MSFT", "NVDA", "TSLA", "SPY", "QQQ", "META", "GOOGL", "AMZN", "AMD"];
  let signals: UltimateSignal[] = [];
  let filterGrade: string = 'all';
  let filterDirection: string = 'all';
  let minScore = 0;
  let q = "";

  onMount(() => {
    // Generate mock signals for demo
    const scorer = new UltimateScorer();
    
    const mockSignals = watchlist.map((symbol, i) => {
      const level = i % 3 === 0 ? 'high' : i % 3 === 1 ? 'medium' : 'low';
      const scores = createMockScores(level);
      const direction = i % 2 === 0 ? 'long' : 'short';
      const basePrice = 100 + (i * 50);
      
      return scorer.calculateUltimateScore(
        symbol,
        '15m',
        scores.v1,
        scores.v2,
        scores.v3,
        scores.v4,
        scores.v5,
        direction,
        {
          entry: basePrice,
          stopLoss: direction === 'long' ? basePrice * 0.98 : basePrice * 1.02,
          tp1: direction === 'long' ? basePrice * 1.04 : basePrice * 0.96,
          tp2: direction === 'long' ? basePrice * 1.06 : basePrice * 0.94,
          tp3: direction === 'long' ? basePrice * 1.10 : basePrice * 0.90
        }
      );
    });

    signals = mockSignals.sort((a, b) => b.totalScore - a.totalScore);
  });

  function getGradeColor(grade: string): string {
    switch(grade) {
      case 'S': return '#FFD700';
      case 'A': return '#26a69a';
      case 'B': return '#00BCD4';
      case 'C': return '#FFA726';
      case 'D': return '#FF5252';
      default: return '#9E9E9E';
    }
  }

  $: filteredSignals = signals
    .filter(s => s.symbol.toLowerCase().includes(q.toLowerCase()))
    .filter(s => filterGrade === 'all' || s.grade === filterGrade)
    .filter(s => filterDirection === 'all' || s.direction === filterDirection)
    .filter(s => s.totalScore >= minScore);
</script>

<style>
  .scanner {
    background: #0b0e11;
    color: #d1d4dc;
    min-height: 100vh;
    padding: 24px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  .title {
    font-size: 32px;
    font-weight: 700;
    margin: 0;
  }

  .filters {
    display: flex;
    gap: 12px;
    margin-bottom: 24px;
    flex-wrap: wrap;
  }

  .filter-input, .filter-select {
    background: #141b22;
    border: 1px solid #1f2933;
    color: #d1d4dc;
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 14px;
  }

  .signal-card {
    background: #141b22;
    border: 1px solid #1f2933;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 16px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .signal-card:hover {
    border-color: #667eea;
    transform: translateY(-2px);
  }

  .signal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .symbol {
    font-size: 24px;
    font-weight: 700;
  }

  .grade-badge {
    font-size: 18px;
    font-weight: 700;
    padding: 6px 14px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.1);
  }

  .score-bar {
    background: rgba(255, 255, 255, 0.1);
    height: 8px;
    border-radius: 4px;
    overflow: hidden;
    margin: 12px 0;
  }

  .score-fill {
    height: 100%;
    background: linear-gradient(90deg, #667eea, #764ba2);
    transition: width 0.3s;
  }

  .components {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 8px;
    margin-top: 16px;
  }

  .component {
    padding: 8px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    text-align: center;
  }

  .component-label {
    font-size: 10px;
    opacity: 0.7;
  }

  .component-value {
    font-size: 14px;
    font-weight: 700;
    margin-top: 4px;
  }
</style>

<div class="scanner">
  <div class="header">
    <h1 class="title">📊 Signal Scanner</h1>
    <div style="font-size: 14px; opacity: 0.7;">
      {filteredSignals.length} signals found
    </div>
  </div>

  <div class="filters">
    <input
      type="text"
      placeholder="Search symbol..."
      bind:value={q}
      class="filter-input"
      style="width: 200px;"
    />
    
    <select bind:value={filterGrade} class="filter-select">
      <option value="all">All Grades</option>
      <option value="S">S Grade</option>
      <option value="A">A Grade</option>
      <option value="B">B Grade</option>
      <option value="C">C Grade</option>
      <option value="D">D Grade</option>
      <option value="F">F Grade</option>
    </select>
    
    <select bind:value={filterDirection} class="filter-select">
      <option value="all">All Directions</option>
      <option value="long">Long Only</option>
      <option value="short">Short Only</option>
    </select>
    
    <div style="display: flex; align-items: center; gap: 8px; background: #141b22; border: 1px solid #1f2933; padding: 10px 14px; border-radius: 8px;">
      <label style="font-size: 14px;">Min Score: {minScore}</label>
      <input
        type="range"
        min="0"
        max="100"
        bind:value={minScore}
        style="width: 150px;"
      />
    </div>
  </div>

  {#each filteredSignals as signal (signal.symbol)}
    <div class="signal-card">
      <div class="signal-header">
        <div>
          <div class="symbol">{signal.symbol}</div>
          <div style="font-size: 14px; opacity: 0.7; margin-top: 4px;">
            {signal.direction.toUpperCase()} • {signal.timeframe} • {signal.action.replace('_', ' ')}
          </div>
        </div>
        <div class="grade-badge" style="color: {getGradeColor(signal.grade)}">
          {signal.grade}
        </div>
      </div>

      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
        <span style="font-size: 13px; opacity: 0.7;">Total Score</span>
        <span style="font-size: 18px; font-weight: 700;">{signal.totalScore}/100</span>
      </div>
      
      <div class="score-bar">
        <div class="score-fill" style="width: {signal.totalScore}%;"></div>
      </div>

      <div class="components">
        <div class="component">
          <div class="component-label">V1 SMC</div>
          <div class="component-value">{signal.v1_smc.total}/25</div>
        </div>
        <div class="component">
          <div class="component-label">V2 Flow</div>
          <div class="component-value">{signal.v2_flow.total}/25</div>
        </div>
        <div class="component">
          <div class="component-label">V3 Auction</div>
          <div class="component-value">{signal.v3_auction.total}/20</div>
        </div>
        <div class="component">
          <div class="component-label">V4 Quantum</div>
          <div class="component-value">{signal.v4_quantum.total}/15</div>
        </div>
        <div class="component">
          <div class="component-label">V5 Psych</div>
          <div class="component-value">{signal.v5_psychology.total}/15</div>
        </div>
      </div>

      <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; margin-top: 16px; padding-top: 16px; border-top: 1px solid #1f2933;">
        <div>
          <div style="font-size: 11px; opacity: 0.6;">ENTRY</div>
          <div style="font-weight: 700;">${signal.entry.toFixed(2)}</div>
        </div>
        <div>
          <div style="font-size: 11px; opacity: 0.6;">STOP</div>
          <div style="font-weight: 700; color: #ef5350;">${signal.stopLoss.toFixed(2)}</div>
        </div>
        <div>
          <div style="font-size: 11px; opacity: 0.6;">TP1</div>
          <div style="font-weight: 700; color: #26a69a;">${signal.takeProfit1.toFixed(2)}</div>
        </div>
      </div>

      <div style="margin-top: 12px; font-size: 12px; opacity: 0.8;">
        <strong>Win Probability:</strong> {signal.winProbability.toFixed(0)}% • 
        <strong>R:R:</strong> {signal.riskReward.toFixed(2)}:1
      </div>
    </div>
  {/each}

  {#if filteredSignals.length === 0}
    <div style="text-align: center; padding: 40px; opacity: 0.6;">
      No signals match your filters
    </div>
  {/if}
</div>
