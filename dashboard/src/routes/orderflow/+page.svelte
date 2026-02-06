<script lang="ts">
  let orderFlowData = {
    delta: { current: 125000, cumulative: 450000, signal: 'bullish' },
    darkPool: { prints: 12, volume: 2500000, signal: 'bullish' },
    options: { pcRatio: 1.45, callVolume: 15000, putVolume: 10345 }
  };
</script>

<div class="orderflow">
  <h1>💹 Order Flow Analysis</h1>
  <p class="subtitle">Analyse du flux d'ordres et liquidité institutionnelle</p>
  
  <div class="metrics-grid">
    <div class="metric-card">
      <h3>📊 Delta Analysis</h3>
      <div class="value">{orderFlowData.delta.current.toLocaleString()}</div>
      <div class="label">Delta Actuel</div>
      <div class="cumulative">Cumulatif: {orderFlowData.delta.cumulative.toLocaleString()}</div>
      <div class="signal" style="color: #26a69a">↗️ Signal Haussier</div>
    </div>
    
    <div class="metric-card">
      <h3>🏊 Dark Pool</h3>
      <div class="value">{orderFlowData.darkPool.prints}</div>
      <div class="label">Prints Détectés</div>
      <div class="cumulative">Volume: {(orderFlowData.darkPool.volume / 1000000).toFixed(2)}M</div>
      <div class="signal" style="color: #26a69a">↗️ Signal Haussier</div>
    </div>
    
    <div class="metric-card">
      <h3>📈 Options Flow</h3>
      <div class="value">{orderFlowData.options.pcRatio.toFixed(2)}</div>
      <div class="label">Put/Call Ratio</div>
      <div class="cumulative">
        Calls: {orderFlowData.options.callVolume.toLocaleString()} | 
        Puts: {orderFlowData.options.putVolume.toLocaleString()}
      </div>
      <div class="signal" style="color: #26a69a">↗️ Calls Dominants</div>
    </div>
  </div>
</div>

<style>
  .orderflow {
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
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .metric-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 2rem;
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
    margin-bottom: 1rem;
  }
  
  .cumulative {
    color: #d1d4dc;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }
  
  .signal {
    font-weight: 600;
  }
</style>
