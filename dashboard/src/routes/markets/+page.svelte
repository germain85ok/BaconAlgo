<script lang="ts">
  import { onMount } from 'svelte';
  
  let marketData = {
    indices: [
      { name: 'S&P 500', symbol: 'SPY', price: 450.25, change: 2.5 },
      { name: 'Nasdaq', symbol: 'QQQ', price: 380.10, change: 3.2 },
      { name: 'Dow Jones', symbol: 'DIA', price: 340.50, change: 1.8 }
    ],
    sectors: [
      { name: 'Technology', performance: 3.5, sentiment: 'bullish' },
      { name: 'Finance', performance: -0.8, sentiment: 'neutral' },
      { name: 'Energy', performance: 2.1, sentiment: 'bullish' },
      { name: 'Healthcare', performance: 1.2, sentiment: 'neutral' }
    ]
  };
</script>

<div class="markets">
  <h1>📊 Vue Marchés</h1>
  <p class="subtitle">Aperçu global des marchés financiers</p>
  
  <section class="section">
    <h2>Indices Majeurs</h2>
    <div class="indices-grid">
      {#each marketData.indices as index}
        <div class="index-card">
          <h3>{index.name}</h3>
          <div class="symbol">{index.symbol}</div>
          <div class="price">${index.price.toFixed(2)}</div>
          <div class="change" class:positive={index.change > 0} class:negative={index.change < 0}>
            {index.change > 0 ? '+' : ''}{index.change.toFixed(2)}%
          </div>
        </div>
      {/each}
    </div>
  </section>
  
  <section class="section">
    <h2>Performance Sectorielle</h2>
    <div class="sectors-list">
      {#each marketData.sectors as sector}
        <div class="sector-item">
          <div class="sector-name">{sector.name}</div>
          <div class="sector-bar">
            <div class="bar-fill" style="width: {Math.abs(sector.performance) * 10}%; background: {sector.performance > 0 ? '#26a69a' : '#ef5350'}"></div>
          </div>
          <div class="sector-perf" class:positive={sector.performance > 0} class:negative={sector.performance < 0}>
            {sector.performance > 0 ? '+' : ''}{sector.performance.toFixed(1)}%
          </div>
        </div>
      {/each}
    </div>
  </section>
</div>

<style>
  .markets {
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
  
  .section {
    margin-bottom: 3rem;
  }
  
  .section h2 {
    color: #ff6b35;
    margin-bottom: 1.5rem;
  }
  
  .indices-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }
  
  .index-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    text-align: center;
  }
  
  .index-card h3 {
    margin: 0 0 0.5rem 0;
    color: #d1d4dc;
  }
  
  .symbol {
    color: #95a5a6;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }
  
  .price {
    font-size: 2rem;
    font-weight: 700;
    color: #ff6b35;
    margin-bottom: 0.5rem;
  }
  
  .change {
    font-size: 1.25rem;
    font-weight: 600;
  }
  
  .positive {
    color: #26a69a;
  }
  
  .negative {
    color: #ef5350;
  }
  
  .sectors-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .sector-item {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1rem 1.5rem;
    display: grid;
    grid-template-columns: 150px 1fr 100px;
    align-items: center;
    gap: 1rem;
  }
  
  .sector-name {
    font-weight: 600;
  }
  
  .sector-bar {
    height: 24px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 4px;
    overflow: hidden;
  }
  
  .bar-fill {
    height: 100%;
  }
  
  .sector-perf {
    text-align: right;
    font-weight: 600;
  }
</style>
