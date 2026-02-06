<script lang="ts">
  import { onMount } from "svelte";
  import { UltimateScorer, createMockScores, type UltimateSignal } from "$lib/scoring/ultimateScore";

  // Mock data for demonstration
  let portfolioValue = 125430.50;
  let dailyPnL = 2340.75;
  let dailyPnLPercent = 1.89;
  let activeSignals: UltimateSignal[] = [];
  let topSignal: UltimateSignal | null = null;

  onMount(() => {
    // Create mock signals for demo
    const scorer = new UltimateScorer();
    
    const highScores = createMockScores('high');
    topSignal = scorer.calculateUltimateScore(
      'NVDA',
      '15m',
      highScores.v1,
      highScores.v2,
      highScores.v3,
      highScores.v4,
      highScores.v5,
      'long',
      { entry: 875.50, stopLoss: 870.00, tp1: 885.00, tp2: 895.00, tp3: 910.00 }
    );

    const mediumScores = createMockScores('medium');
    const signal2 = scorer.calculateUltimateScore(
      'TSLA',
      '5m',
      mediumScores.v1,
      mediumScores.v2,
      mediumScores.v3,
      mediumScores.v4,
      mediumScores.v5,
      'short',
      { entry: 242.50, stopLoss: 245.00, tp1: 238.00, tp2: 235.00, tp3: 230.00 }
    );

    activeSignals = [topSignal, signal2];
  });

  function getGradeColor(grade: string): string {
    switch(grade) {
      case 'S': return '#FFD700'; // Gold
      case 'A': return '#26a69a'; // Green
      case 'B': return '#00BCD4'; // Cyan
      case 'C': return '#FFA726'; // Orange
      case 'D': return '#FF5252'; // Red
      default: return '#9E9E9E'; // Gray
    }
  }
</script>

<style>
  .dashboard {
    background: #0b0e11;
    color: #d1d4dc;
    min-height: 100vh;
    padding: 24px;
  }

  .header {
    margin-bottom: 32px;
  }

  .title {
    font-size: 32px;
    font-weight: 700;
    margin: 0 0 8px 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .subtitle {
    font-size: 14px;
    opacity: 0.7;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
    margin-bottom: 24px;
  }

  .card {
    background: #141b22;
    border: 1px solid #1f2933;
    border-radius: 12px;
    padding: 20px;
  }

  .card-title {
    font-size: 14px;
    font-weight: 600;
    opacity: 0.7;
    margin: 0 0 12px 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .metric {
    font-size: 32px;
    font-weight: 700;
    margin: 0;
  }

  .metric-change {
    font-size: 14px;
    margin-top: 4px;
  }

  .positive {
    color: #26a69a;
  }

  .negative {
    color: #ef5350;
  }

  .signal-card {
    background: #141b22;
    border: 1px solid #1f2933;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 16px;
  }

  .signal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .signal-symbol {
    font-size: 24px;
    font-weight: 700;
  }

  .grade-badge {
    font-size: 20px;
    font-weight: 700;
    padding: 8px 16px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.1);
  }

  .signal-stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    margin-top: 16px;
  }

  .stat {
    padding: 8px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
  }

  .stat-label {
    font-size: 11px;
    opacity: 0.6;
    text-transform: uppercase;
  }

  .stat-value {
    font-size: 16px;
    font-weight: 600;
    margin-top: 4px;
  }

  .nav-links {
    display: flex;
    gap: 12px;
    margin-top: 24px;
  }

  .nav-link {
    padding: 12px 20px;
    background: #1f2933;
    border: 1px solid #2d3748;
    border-radius: 8px;
    color: #d1d4dc;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.2s;
  }

  .nav-link:hover {
    background: #2d3748;
    border-color: #667eea;
  }
</style>

<div class="dashboard">
  <div class="header">
    <h1 class="title">🥓 BaconAlgo Ultimate Dashboard</h1>
    <p class="subtitle">V1-V5 Complete Trading Platform | Smart Money + Order Flow + Quantum + Psychology</p>
  </div>

  <div class="grid">
    <div class="card">
      <h3 class="card-title">Portfolio Value</h3>
      <p class="metric">${portfolioValue.toLocaleString('en-US', {minimumFractionDigits: 2})}</p>
    </div>

    <div class="card">
      <h3 class="card-title">Today's P&L</h3>
      <p class="metric {dailyPnL >= 0 ? 'positive' : 'negative'}">
        {dailyPnL >= 0 ? '+' : ''}{dailyPnL >= 0 ? '$' : '-$'}{Math.abs(dailyPnL).toLocaleString('en-US', {minimumFractionDigits: 2})}
      </p>
      <p class="metric-change {dailyPnLPercent >= 0 ? 'positive' : 'negative'}">
        {dailyPnLPercent >= 0 ? '+' : ''}{dailyPnLPercent.toFixed(2)}%
      </p>
    </div>

    <div class="card">
      <h3 class="card-title">Active Signals</h3>
      <p class="metric">{activeSignals.length}</p>
      <p class="metric-change">
        {activeSignals.filter(s => s.grade === 'S' || s.grade === 'A').length} Premium Signals
      </p>
    </div>
  </div>

  {#if topSignal}
  <div class="signal-card">
    <div class="signal-header">
      <div>
        <div class="signal-symbol">{topSignal.symbol} - {topSignal.direction.toUpperCase()}</div>
        <div style="opacity: 0.7; font-size: 14px; margin-top: 4px;">{topSignal.timeframe} Timeframe</div>
      </div>
      <div class="grade-badge" style="color: {getGradeColor(topSignal.grade)}">
        {topSignal.grade} Grade
      </div>
    </div>

    <div style="margin: 16px 0;">
      <div style="font-size: 14px; opacity: 0.7; margin-bottom: 8px;">Total Score: {topSignal.totalScore}/100</div>
      <div style="background: rgba(255,255,255,0.1); height: 8px; border-radius: 4px; overflow: hidden;">
        <div style="background: linear-gradient(90deg, #667eea, #764ba2); height: 100%; width: {topSignal.totalScore}%; transition: width 0.3s;"></div>
      </div>
    </div>

    <div style="margin: 16px 0;">
      <div style="font-size: 13px; opacity: 0.8; margin-bottom: 8px;">Component Scores:</div>
      <div style="display: grid; grid-template-columns: repeat(5, 1fr); gap: 8px;">
        <div style="padding: 8px; background: rgba(102, 126, 234, 0.2); border-radius: 6px; text-align: center;">
          <div style="font-size: 11px; opacity: 0.7;">V1 SMC</div>
          <div style="font-size: 16px; font-weight: 700;">{topSignal.v1_smc.total}/25</div>
        </div>
        <div style="padding: 8px; background: rgba(118, 75, 162, 0.2); border-radius: 6px; text-align: center;">
          <div style="font-size: 11px; opacity: 0.7;">V2 Flow</div>
          <div style="font-size: 16px; font-weight: 700;">{topSignal.v2_flow.total}/25</div>
        </div>
        <div style="padding: 8px; background: rgba(38, 166, 154, 0.2); border-radius: 6px; text-align: center;">
          <div style="font-size: 11px; opacity: 0.7;">V3 Auction</div>
          <div style="font-size: 16px; font-weight: 700;">{topSignal.v3_auction.total}/20</div>
        </div>
        <div style="padding: 8px; background: rgba(255, 167, 38, 0.2); border-radius: 6px; text-align: center;">
          <div style="font-size: 11px; opacity: 0.7;">V4 Quantum</div>
          <div style="font-size: 16px; font-weight: 700;">{topSignal.v4_quantum.total}/15</div>
        </div>
        <div style="padding: 8px; background: rgba(0, 188, 212, 0.2); border-radius: 6px; text-align: center;">
          <div style="font-size: 11px; opacity: 0.7;">V5 Psych</div>
          <div style="font-size: 16px; font-weight: 700;">{topSignal.v5_psychology.total}/15</div>
        </div>
      </div>
    </div>

    <div class="signal-stats">
      <div class="stat">
        <div class="stat-label">Entry</div>
        <div class="stat-value">${topSignal.entry.toFixed(2)}</div>
      </div>
      <div class="stat">
        <div class="stat-label">Stop Loss</div>
        <div class="stat-value negative">${topSignal.stopLoss.toFixed(2)}</div>
      </div>
      <div class="stat">
        <div class="stat-label">TP1</div>
        <div class="stat-value positive">${topSignal.takeProfit1.toFixed(2)}</div>
      </div>
      <div class="stat">
        <div class="stat-label">R:R Ratio</div>
        <div class="stat-value">{topSignal.riskReward.toFixed(2)}:1</div>
      </div>
      <div class="stat">
        <div class="stat-label">Win Prob</div>
        <div class="stat-value">{topSignal.winProbability.toFixed(0)}%</div>
      </div>
      <div class="stat">
        <div class="stat-label">Action</div>
        <div class="stat-value" style="font-size: 12px;">{topSignal.action.replace('_', ' ')}</div>
      </div>
    </div>

    <div style="margin-top: 16px; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 6px;">
      <div style="font-size: 12px; opacity: 0.7; margin-bottom: 8px;">REASONS:</div>
      {#each topSignal.reasons as reason}
        <div style="font-size: 13px; margin: 4px 0;">• {reason}</div>
      {/each}
    </div>
  </div>
  {/if}

  <div class="nav-links">
    <a href="/scanner" class="nav-link">📊 Scanner</a>
    <a href="/markets" class="nav-link">🌍 Markets</a>
    <a href="/orderflow" class="nav-link">💹 Order Flow</a>
    <a href="/auction" class="nav-link">📈 Auction Theory</a>
    <a href="/quantum" class="nav-link">⚛️ Quantum</a>
    <a href="/psychology" class="nav-link">🧠 Psychology</a>
    <a href="/risk" class="nav-link">🛡️ Risk</a>
  </div>
</div>
