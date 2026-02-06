<script lang="ts">
  let symbol = 'AAPL';
  
  // Mock order flow data
  let delta = 12450;
  let cumulativeDelta = 45230;
  let buyVolume = 245670;
  let sellVolume = 233220;
  let buyPressure = 51.3;
  
  let darkPoolData = {
    dix: 0.62,
    totalPrints: 23,
    volume: 1234567,
    trend: 'bullish' as const
  };
  
  let optionsData = {
    putCallRatio: 0.78,
    callPremium: 12500000,
    putPremium: 8900000,
    unusualActivity: 5
  };
</script>

<style>
  .orderflow {
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
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
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
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 12px 0;
  }

  .metric-label {
    opacity: 0.7;
  }

  .metric-value {
    font-size: 18px;
    font-weight: 700;
  }

  .positive {
    color: #26a69a;
  }

  .negative {
    color: #ef5350;
  }

  .delta-bar {
    background: #1f2933;
    height: 40px;
    border-radius: 8px;
    overflow: hidden;
    position: relative;
    margin-top: 16px;
  }

  .delta-fill {
    position: absolute;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 14px;
  }
</style>

<div class="orderflow">
  <h1 class="title">💹 Order Flow Analysis - {symbol}</h1>

  <div class="grid">
    <div class="card">
      <h3 class="card-title">📊 Volume Delta</h3>
      
      <div class="metric">
        <span class="metric-label">Current Delta</span>
        <span class="metric-value {delta >= 0 ? 'positive' : 'negative'}">
          {delta >= 0 ? '+' : ''}{delta.toLocaleString()}
        </span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Cumulative Delta</span>
        <span class="metric-value {cumulativeDelta >= 0 ? 'positive' : 'negative'}">
          {cumulativeDelta >= 0 ? '+' : ''}{cumulativeDelta.toLocaleString()}
        </span>
      </div>
      
      <div class="delta-bar">
        <div class="delta-fill" style="background: #26a69a; width: {buyPressure}%; left: 0;">
          {buyVolume.toLocaleString()} BUY
        </div>
        <div class="delta-fill" style="background: #ef5350; width: {100 - buyPressure}%; right: 0;">
          {sellVolume.toLocaleString()} SELL
        </div>
      </div>
      
      <div style="text-align: center; margin-top: 8px; opacity: 0.7; font-size: 13px;">
        Buy Pressure: {buyPressure.toFixed(1)}%
      </div>
    </div>

    <div class="card">
      <h3 class="card-title">🕶️ Dark Pool Activity</h3>
      
      <div class="metric">
        <span class="metric-label">DIX (Dark Index)</span>
        <span class="metric-value">{darkPoolData.dix.toFixed(2)}</span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Total Prints</span>
        <span class="metric-value">{darkPoolData.totalPrints}</span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Dark Volume</span>
        <span class="metric-value">{(darkPoolData.volume / 1000000).toFixed(2)}M</span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Trend</span>
        <span class="metric-value {darkPoolData.trend === 'bullish' ? 'positive' : 'negative'}">
          {darkPoolData.trend.toUpperCase()} ⬆️
        </span>
      </div>
    </div>

    <div class="card">
      <h3 class="card-title">📈 Options Flow</h3>
      
      <div class="metric">
        <span class="metric-label">Put/Call Ratio</span>
        <span class="metric-value">{optionsData.putCallRatio.toFixed(2)}</span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Call Premium</span>
        <span class="metric-value positive">${(optionsData.callPremium / 1000000).toFixed(1)}M</span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Put Premium</span>
        <span class="metric-value negative">${(optionsData.putPremium / 1000000).toFixed(1)}M</span>
      </div>
      
      <div class="metric">
        <span class="metric-label">Unusual Activity</span>
        <span class="metric-value">{optionsData.unusualActivity} alerts</span>
      </div>
      
      <div style="margin-top: 16px; padding: 12px; background: rgba(38, 166, 154, 0.2); border-radius: 6px;">
        <div style="font-size: 13px; font-weight: 600;">Smart Money Signal</div>
        <div style="font-size: 20px; font-weight: 700; color: #26a69a; margin-top: 4px;">
          BULLISH
        </div>
      </div>
    </div>
  </div>

  <div class="card">
    <h3 class="card-title">🐋 Whale Activity</h3>
    <div style="padding: 20px; text-align: center; opacity: 0.6;">
      Live whale tracking coming soon...
    </div>
  </div>
</div>
