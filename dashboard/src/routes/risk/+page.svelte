<script lang="ts">
  let portfolioValue = 125430.50;
  let dailyRisk = 2508.61; // 2% of portfolio
  let maxDrawdown = 5;
  let currentDrawdown = 1.2;
  let sharpeRatio = 2.4;
  let winRate = 68.5;
  
  let positions = [
    { symbol: 'NVDA', size: 100, entry: 875.50, current: 880.25, pnl: 475.00, risk: 1.2 },
    { symbol: 'TSLA', size: 50, entry: 242.50, current: 240.75, pnl: -87.50, risk: 0.8 }
  ];
  
  let stressTests = [
    { scenario: 'Flash Crash -10%', impact: -12543.05, worstCase: true },
    { scenario: 'Volatility Spike +50%', impact: -6271.53, worstCase: false },
    { scenario: 'Market Rally +5%', impact: 6271.53, worstCase: false },
    { scenario: 'Sector Rotation', impact: -3135.76, worstCase: false }
  ];
</script>

<style>
  .risk {
    background: #0b0e11;
    color: #d1d4dc;
    min-height: 100vh;
    padding: 24px;
  }

  .title {
    font-size: 32px;
    font-weight: 700;
    margin: 0 0 24px 0;
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
    font-size: 18px;
    font-weight: 600;
    margin: 0 0 16px 0;
  }

  .metric {
    font-size: 28px;
    font-weight: 700;
    margin: 8px 0;
  }

  .metric-label {
    font-size: 13px;
    opacity: 0.7;
    text-transform: uppercase;
  }

  .progress-bar {
    background: rgba(255, 255, 255, 0.1);
    height: 24px;
    border-radius: 12px;
    overflow: hidden;
    margin: 12px 0;
  }

  .progress-fill {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 12px;
  }

  .position-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 12px;
    padding: 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    margin: 8px 0;
  }

  .kill-switch {
    background: rgba(239, 83, 80, 0.2);
    border: 2px solid #ef5350;
    border-radius: 12px;
    padding: 24px;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s;
  }

  .kill-switch:hover {
    background: rgba(239, 83, 80, 0.3);
    transform: scale(1.02);
  }
</style>

<div class="risk">
  <h1 class="title">🛡️ Risk Management</h1>

  <div class="grid">
    <div class="card">
      <div class="metric-label">Portfolio Value</div>
      <div class="metric">${portfolioValue.toLocaleString('en-US', {minimumFractionDigits: 2})}</div>
    </div>

    <div class="card">
      <div class="metric-label">Daily Risk (2%)</div>
      <div class="metric" style="color: #FFA726;">${dailyRisk.toLocaleString('en-US', {minimumFractionDigits: 2})}</div>
    </div>

    <div class="card">
      <div class="metric-label">Sharpe Ratio</div>
      <div class="metric" style="color: #26a69a;">{sharpeRatio.toFixed(2)}</div>
    </div>

    <div class="card">
      <div class="metric-label">Win Rate</div>
      <div class="metric" style="color: #26a69a;">{winRate.toFixed(1)}%</div>
    </div>
  </div>

  <div class="card">
    <h3 class="card-title">📉 Drawdown Monitor</h3>
    
    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 16px;">
      <div>
        <div class="metric-label">Current Drawdown</div>
        <div class="metric" style="color: {currentDrawdown > 3 ? '#ef5350' : '#26a69a'};">
          {currentDrawdown.toFixed(1)}%
        </div>
      </div>
      <div>
        <div class="metric-label">Max Allowed</div>
        <div class="metric" style="color: #FFA726;">
          {maxDrawdown.toFixed(1)}%
        </div>
      </div>
    </div>
    
    <div class="progress-bar">
      <div class="progress-fill" style="background: {currentDrawdown / maxDrawdown > 0.8 ? '#ef5350' : '#26a69a'}; width: {(currentDrawdown / maxDrawdown) * 100}%;">
        {currentDrawdown.toFixed(1)}% / {maxDrawdown}%
      </div>
    </div>
    
    {#if currentDrawdown / maxDrawdown > 0.8}
      <div style="margin-top: 12px; padding: 12px; background: rgba(239, 83, 80, 0.2); border-radius: 6px; color: #ef5350;">
        ⚠️ Warning: Approaching maximum drawdown limit!
      </div>
    {/if}
  </div>

  <div class="card">
    <h3 class="card-title">📊 Active Positions</h3>
    
    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 12px; padding: 12px; font-size: 12px; opacity: 0.7; font-weight: 600;">
      <div>SYMBOL</div>
      <div>ENTRY</div>
      <div>CURRENT</div>
      <div>P&L</div>
    </div>
    
    {#each positions as pos}
      <div class="position-row">
        <div style="font-weight: 700;">{pos.symbol}</div>
        <div>${pos.entry.toFixed(2)}</div>
        <div>${pos.current.toFixed(2)}</div>
        <div style="color: {pos.pnl >= 0 ? '#26a69a' : '#ef5350'}; font-weight: 700;">
          {pos.pnl >= 0 ? '+' : ''}{pos.pnl >= 0 ? '$' : '-$'}{Math.abs(pos.pnl).toFixed(2)}
        </div>
      </div>
    {/each}
  </div>

  <div class="card">
    <h3 class="card-title">⚡ Stress Testing</h3>
    
    {#each stressTests as test}
      <div style="padding: 12px; margin: 8px 0; background: rgba(255, 255, 255, 0.05); border-radius: 6px;">
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <div>
            <div style="font-weight: 600;">{test.scenario}</div>
            {#if test.worstCase}
              <div style="font-size: 11px; color: #ef5350; margin-top: 2px;">⚠️ WORST CASE</div>
            {/if}
          </div>
          <div style="font-size: 18px; font-weight: 700; color: {test.impact >= 0 ? '#26a69a' : '#ef5350'};">
            {test.impact >= 0 ? '+' : ''}{test.impact >= 0 ? '$' : '-$'}{Math.abs(test.impact).toLocaleString('en-US', {minimumFractionDigits: 2})}
          </div>
        </div>
      </div>
    {/each}
  </div>

  <div class="kill-switch">
    <div style="font-size: 48px; margin-bottom: 12px;">🚨</div>
    <div style="font-size: 24px; font-weight: 700; margin-bottom: 8px;">KILL SWITCH</div>
    <div style="opacity: 0.8; font-size: 14px;">Close all positions & cancel all orders</div>
  </div>
</div>
