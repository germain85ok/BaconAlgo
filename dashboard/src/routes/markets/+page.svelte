<script lang="ts">
  import { onMount } from "svelte";

  let symbols = ['SPY', 'QQQ', 'AAPL', 'TSLA', 'NVDA', 'MSFT', 'GOOGL', 'AMZN', 'META', 'NFLX'];
  let markets = [
    { symbol: 'SPY', name: 'S&P 500', price: 495.23, change: 2.34, changePercent: 0.47, volume: 85234567 },
    { symbol: 'QQQ', name: 'Nasdaq 100', price: 412.56, change: -1.23, changePercent: -0.30, volume: 54321098 },
    { symbol: 'DIA', name: 'Dow Jones', price: 385.67, change: 1.12, changePercent: 0.29, volume: 32145678 },
    { symbol: 'IWM', name: 'Russell 2000', price: 198.45, change: -0.45, changePercent: -0.23, volume: 28765432 }
  ];

  let crypto = [
    { symbol: 'BTC-USD', name: 'Bitcoin', price: 65432.10, change: 1234.50, changePercent: 1.92 },
    { symbol: 'ETH-USD', name: 'Ethereum', price: 3456.78, change: -123.45, changePercent: -3.45 },
    { symbol: 'SOL-USD', name: 'Solana', price: 123.45, change: 5.67, changePercent: 4.81 }
  ];

  let sectors = [
    { name: 'Technology', change: 0.75 },
    { name: 'Finance', change: 0.45 },
    { name: 'Healthcare', change: -0.32 },
    { name: 'Energy', change: 1.23 },
    { name: 'Consumer', change: -0.15 },
    { name: 'Utilities', change: 0.12 }
  ];
</script>

<style>
  .markets {
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
  }

  .section {
    margin-bottom: 32px;
  }

  .section-title {
    font-size: 20px;
    font-weight: 600;
    margin-bottom: 16px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
  }

  .market-card {
    background: #141b22;
    border: 1px solid #1f2933;
    border-radius: 12px;
    padding: 20px;
  }

  .market-symbol {
    font-size: 18px;
    font-weight: 700;
  }

  .market-name {
    font-size: 12px;
    opacity: 0.6;
    margin-top: 2px;
  }

  .market-price {
    font-size: 24px;
    font-weight: 700;
    margin: 12px 0 4px 0;
  }

  .market-change {
    font-size: 14px;
  }

  .positive {
    color: #26a69a;
  }

  .negative {
    color: #ef5350;
  }

  .sector-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .sector-card {
    background: #141b22;
    border: 1px solid #1f2933;
    border-radius: 8px;
    padding: 16px;
    text-align: center;
  }
</style>

<div class="markets">
  <div class="header">
    <h1 class="title">🌍 Markets Overview</h1>
  </div>

  <div class="section">
    <h2 class="section-title">📈 US Indices</h2>
    <div class="grid">
      {#each markets as market}
        <div class="market-card">
          <div class="market-symbol">{market.symbol}</div>
          <div class="market-name">{market.name}</div>
          <div class="market-price">${market.price.toFixed(2)}</div>
          <div class="market-change {market.change >= 0 ? 'positive' : 'negative'}">
            {market.change >= 0 ? '+' : ''}{market.change.toFixed(2)} ({market.changePercent >= 0 ? '+' : ''}{market.changePercent.toFixed(2)}%)
          </div>
          <div style="font-size: 11px; opacity: 0.5; margin-top: 8px;">
            Vol: {(market.volume / 1000000).toFixed(1)}M
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div class="section">
    <h2 class="section-title">₿ Crypto</h2>
    <div class="grid">
      {#each crypto as coin}
        <div class="market-card">
          <div class="market-symbol">{coin.symbol}</div>
          <div class="market-name">{coin.name}</div>
          <div class="market-price">${coin.price.toLocaleString()}</div>
          <div class="market-change {coin.change >= 0 ? 'positive' : 'negative'}">
            {coin.change >= 0 ? '+' : ''}{coin.change.toFixed(2)} ({coin.changePercent >= 0 ? '+' : ''}{coin.changePercent.toFixed(2)}%)
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div class="section">
    <h2 class="section-title">🔥 Sector Heatmap</h2>
    <div class="sector-grid">
      {#each sectors as sector}
        <div class="sector-card" style="background: {sector.change > 0 ? 'rgba(38, 166, 154, 0.2)' : 'rgba(239, 83, 80, 0.2)'}">
          <div style="font-size: 14px; font-weight: 600;">{sector.name}</div>
          <div class="{sector.change >= 0 ? 'positive' : 'negative'}" style="font-size: 20px; font-weight: 700; margin-top: 8px;">
            {sector.change >= 0 ? '+' : ''}{sector.change.toFixed(2)}%
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
