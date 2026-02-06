<script lang="ts">
  let symbol = 'SPY';
  
  let volumeProfile = {
    poc: 495.23, // Point of Control
    vah: 497.50, // Value Area High
    val: 493.00, // Value Area Low
    hvn: [495.20, 494.80, 496.10], // High Volume Nodes
    lvn: [492.50, 498.00] // Low Volume Nodes
  };
  
  let vwap = {
    standard: 495.15,
    upper1: 496.20,
    lower1: 494.10,
    anchored: {
      daily: 495.15,
      weekly: 494.85,
      monthly: 493.50
    }
  };
  
  let currentPrice = 495.67;
</script>

<style>
  .auction {
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

  .level-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    margin: 8px 0;
  }

  .level-label {
    font-weight: 600;
  }

  .level-price {
    font-size: 18px;
    font-weight: 700;
  }

  .chart-container {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    padding: 20px;
    margin-top: 16px;
    min-height: 300px;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.6;
  }
</style>

<div class="auction">
  <h1 class="title">📈 Auction Theory - {symbol}</h1>

  <div style="margin-bottom: 24px; padding: 16px; background: #141b22; border: 1px solid #1f2933; border-radius: 12px;">
    <div style="font-size: 14px; opacity: 0.7; margin-bottom: 8px;">Current Price</div>
    <div style="font-size: 36px; font-weight: 700;">${currentPrice.toFixed(2)}</div>
  </div>

  <div class="grid">
    <div class="card">
      <h3 class="card-title">📊 Volume Profile</h3>
      
      <div class="level-row" style="border: 2px solid #667eea;">
        <span class="level-label">POC (Point of Control)</span>
        <span class="level-price" style="color: #667eea;">${volumeProfile.poc.toFixed(2)}</span>
      </div>
      
      <div class="level-row">
        <span class="level-label">VAH (Value Area High)</span>
        <span class="level-price">${volumeProfile.vah.toFixed(2)}</span>
      </div>
      
      <div class="level-row">
        <span class="level-label">VAL (Value Area Low)</span>
        <span class="level-price">${volumeProfile.val.toFixed(2)}</span>
      </div>
      
      <div style="margin-top: 20px;">
        <div style="font-size: 13px; opacity: 0.7; margin-bottom: 8px;">High Volume Nodes (HVN)</div>
        {#each volumeProfile.hvn as hvn}
          <div style="padding: 8px; background: rgba(38, 166, 154, 0.2); border-radius: 4px; margin: 4px 0;">
            ${hvn.toFixed(2)}
          </div>
        {/each}
      </div>
      
      <div style="margin-top: 16px;">
        <div style="font-size: 13px; opacity: 0.7; margin-bottom: 8px;">Low Volume Nodes (LVN)</div>
        {#each volumeProfile.lvn as lvn}
          <div style="padding: 8px; background: rgba(239, 83, 80, 0.2); border-radius: 4px; margin: 4px 0;">
            ${lvn.toFixed(2)}
          </div>
        {/each}
      </div>
    </div>

    <div class="card">
      <h3 class="card-title">📉 VWAP Levels</h3>
      
      <div class="level-row" style="border: 2px solid #26a69a;">
        <span class="level-label">Standard VWAP</span>
        <span class="level-price" style="color: #26a69a;">${vwap.standard.toFixed(2)}</span>
      </div>
      
      <div class="level-row">
        <span class="level-label">Upper Band (+1σ)</span>
        <span class="level-price">${vwap.upper1.toFixed(2)}</span>
      </div>
      
      <div class="level-row">
        <span class="level-label">Lower Band (-1σ)</span>
        <span class="level-price">${vwap.lower1.toFixed(2)}</span>
      </div>
      
      <div style="margin-top: 20px;">
        <div style="font-size: 13px; opacity: 0.7; margin-bottom: 8px;">Anchored VWAP</div>
        
        <div class="level-row">
          <span>Daily VWAP</span>
          <span style="font-weight: 700;">${vwap.anchored.daily.toFixed(2)}</span>
        </div>
        
        <div class="level-row">
          <span>Weekly VWAP</span>
          <span style="font-weight: 700;">${vwap.anchored.weekly.toFixed(2)}</span>
        </div>
        
        <div class="level-row">
          <span>Monthly VWAP</span>
          <span style="font-weight: 700;">${vwap.anchored.monthly.toFixed(2)}</span>
        </div>
      </div>
    </div>
  </div>

  <div class="card" style="margin-top: 20px;">
    <h3 class="card-title">📊 Volume Profile Chart</h3>
    <div class="chart-container">
      Volume Profile visualization coming soon...
    </div>
  </div>

  <div class="card" style="margin-top: 20px;">
    <h3 class="card-title">👣 Footprint Chart</h3>
    <div class="chart-container">
      Footprint chart with buy/sell imbalance coming soon...
    </div>
  </div>
</div>
