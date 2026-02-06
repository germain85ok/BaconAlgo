<script lang="ts">
  let auctionData = {
    poc: 450.25,
    vah: 455.80,
    val: 445.10,
    vwap: 451.50
  };
</script>

<div class="auction">
  <h1>⚖️ Auction Market Theory</h1>
  <p class="subtitle">Volume Profile et théorie des enchères</p>
  
  <div class="metrics-grid">
    <div class="metric-card">
      <h3>📍 Point of Control (POC)</h3>
      <div class="value">${auctionData.poc.toFixed(2)}</div>
      <div class="label">Zone de plus haut volume</div>
    </div>
    
    <div class="metric-card">
      <h3>📈 Value Area High (VAH)</h3>
      <div class="value">${auctionData.vah.toFixed(2)}</div>
      <div class="label">Limite supérieure value area</div>
    </div>
    
    <div class="metric-card">
      <h3>📉 Value Area Low (VAL)</h3>
      <div class="value">${auctionData.val.toFixed(2)}</div>
      <div class="label">Limite inférieure value area</div>
    </div>
    
    <div class="metric-card">
      <h3>💹 VWAP</h3>
      <div class="value">${auctionData.vwap.toFixed(2)}</div>
      <div class="label">Volume Weighted Average Price</div>
    </div>
  </div>
</div>

<style>
  .auction {
    max-width: 1400px;
    margin: 0 auto;
  }
  
  h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    color: #ff6b35;
  }
  
  .subtitle {
    color: #95a5a6;
    margin: 0 0 2rem 0;
  }
  
  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
  }
  
  .metric-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
  }
  
  .metric-card h3 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
  }
  
  .value {
    font-size: 2.5rem;
    font-weight: 700;
    color: #ff6b35;
    margin-bottom: 0.5rem;
  }
  
  .label {
    color: #95a5a6;
    font-size: 0.875rem;
  }
</style>
