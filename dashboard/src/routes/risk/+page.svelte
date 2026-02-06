<script lang="ts">
  let riskData = {
    portfolioValue: 125430.50,
    totalRisk: 2.5,
    maxDrawdown: -5.2,
    sharpeRatio: 2.34,
    positions: [
      { symbol: 'AAPL', size: 25000, risk: 1.2, stopLoss: 180.50 },
      { symbol: 'TSLA', size: 15000, risk: 2.8, stopLoss: 245.30 },
      { symbol: 'NVDA', size: 20000, risk: 1.5, stopLoss: 520.00 }
    ]
  };
</script>

<div class="risk">
  <h1>🛡️ Gestion du Risque</h1>
  <p class="subtitle">Tableau de bord de gestion des risques</p>
  
  <div class="metrics">
    <div class="metric-card">
      <div class="metric-label">Valeur Portefeuille</div>
      <div class="metric-value">${riskData.portfolioValue.toLocaleString('fr-FR', { minimumFractionDigits: 2 })}</div>
    </div>
    
    <div class="metric-card">
      <div class="metric-label">Risque Total</div>
      <div class="metric-value" style="color: {riskData.totalRisk > 5 ? '#ef5350' : '#f9ca24'}">{riskData.totalRisk}%</div>
    </div>
    
    <div class="metric-card">
      <div class="metric-label">Max Drawdown</div>
      <div class="metric-value" style="color: #ef5350">{riskData.maxDrawdown}%</div>
    </div>
    
    <div class="metric-card">
      <div class="metric-label">Sharpe Ratio</div>
      <div class="metric-value" style="color: #26a69a">{riskData.sharpeRatio.toFixed(2)}</div>
    </div>
  </div>
  
  <section class="positions-section">
    <h2>Positions Actives</h2>
    <div class="positions-table">
      <div class="table-header">
        <div>Symbole</div>
        <div>Taille</div>
        <div>Risque</div>
        <div>Stop Loss</div>
      </div>
      {#each riskData.positions as position}
        <div class="table-row">
          <div class="symbol">{position.symbol}</div>
          <div>${position.size.toLocaleString()}</div>
          <div class="risk-value" style="color: {position.risk > 2 ? '#ef5350' : '#f9ca24'}">{position.risk}%</div>
          <div>${position.stopLoss.toFixed(2)}</div>
        </div>
      {/each}
    </div>
  </section>
</div>

<style>
  .risk {
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
  
  .metrics {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
  }
  
  .metric-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    text-align: center;
  }
  
  .metric-label {
    color: #95a5a6;
    font-size: 0.875rem;
    margin-bottom: 0.75rem;
  }
  
  .metric-value {
    font-size: 2rem;
    font-weight: 700;
    color: #ff6b35;
  }
  
  .positions-section h2 {
    color: #ff6b35;
    margin-bottom: 1.5rem;
  }
  
  .positions-table {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    overflow: hidden;
  }
  
  .table-header,
  .table-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 1rem;
    padding: 1rem 1.5rem;
  }
  
  .table-header {
    background: rgba(255, 107, 53, 0.1);
    font-weight: 600;
    color: #ff6b35;
  }
  
  .table-row {
    border-top: 1px solid rgba(255, 107, 53, 0.1);
  }
  
  .symbol {
    font-weight: 700;
  }
  
  .risk-value {
    font-weight: 600;
  }
</style>
