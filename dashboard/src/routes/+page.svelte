<script lang="ts">
  import { onMount } from "svelte";
  import { calculateUltimateScore, getGradeColor, getSignalColor } from '$lib/scoring';
  
  let topSignals: Array<{
    symbol: string;
    score: number;
    grade: string;
    signal: string;
    win_probability: number;
  }> = [];
  
  let portfolioStats = {
    totalValue: 125430.50,
    dailyPnl: 3240.75,
    dailyPnlPercent: 2.65,
    winRate: 68.5,
    sharpeRatio: 2.34,
    activeTrades: 8
  };
  
  onMount(async () => {
    // Simulate some top signals
    const symbols = ['AAPL', 'TSLA', 'NVDA', 'MSFT', 'GOOGL'];
    
    topSignals = symbols.map(symbol => {
      const v1 = 70 + Math.random() * 30;
      const v2 = 65 + Math.random() * 35;
      const v3 = 60 + Math.random() * 40;
      const v4 = 68 + Math.random() * 32;
      const v5 = 72 + Math.random() * 28;
      const v6 = 75 + Math.random() * 25;
      
      const result = calculateUltimateScore(
        v1, v2, v3, v4, v5, v6,
        'bullish', 'bullish', 'neutral', 'bullish', 'neutral', 'healthy'
      );
      
      return {
        symbol,
        score: result.score,
        grade: result.grade,
        signal: result.signal,
        win_probability: result.win_probability
      };
    }).sort((a, b) => b.score - a.score);
  });
</script>

<div class="dashboard">
  <header class="header">
    <h1>🥓 BaconAlgo Dashboard</h1>
    <p class="subtitle">Système d'analyse V1-V5 Trading</p>
  </header>
  
  <div class="stats-grid">
    <div class="stat-card">
      <div class="stat-label">Valeur Portefeuille</div>
      <div class="stat-value">${portfolioStats.totalValue.toLocaleString('fr-FR', { minimumFractionDigits: 2 })}</div>
    </div>
    
    <div class="stat-card" class:positive={portfolioStats.dailyPnl > 0}>
      <div class="stat-label">P&L Journalier</div>
      <div class="stat-value">
        {portfolioStats.dailyPnl > 0 ? '+' : ''}{portfolioStats.dailyPnl.toLocaleString('fr-FR', { minimumFractionDigits: 2 })}
        <span class="stat-percent">({portfolioStats.dailyPnlPercent > 0 ? '+' : ''}{portfolioStats.dailyPnlPercent}%)</span>
      </div>
    </div>
    
    <div class="stat-card">
      <div class="stat-label">Taux de Réussite</div>
      <div class="stat-value">{portfolioStats.winRate}%</div>
    </div>
    
    <div class="stat-card">
      <div class="stat-label">Sharpe Ratio</div>
      <div class="stat-value">{portfolioStats.sharpeRatio.toFixed(2)}</div>
    </div>
    
    <div class="stat-card">
      <div class="stat-label">Trades Actifs</div>
      <div class="stat-value">{portfolioStats.activeTrades}</div>
    </div>
  </div>
  
  <section class="top-signals">
    <h2>🔥 Top Signaux (Score Ultimate)</h2>
    
    <div class="signals-table">
      <div class="table-header">
        <div>Symbole</div>
        <div>Score</div>
        <div>Grade</div>
        <div>Signal</div>
        <div>Win Prob.</div>
      </div>
      
      {#each topSignals as signal}
        <div class="table-row">
          <div class="symbol">{signal.symbol}</div>
          <div class="score">
            <div class="score-bar" style="width: {signal.score}%; background: {getGradeColor(signal.grade)}"></div>
            <span class="score-text">{signal.score.toFixed(1)}</span>
          </div>
          <div class="grade" style="color: {getGradeColor(signal.grade)}">{signal.grade}</div>
          <div class="signal" style="color: {getSignalColor(signal.signal)}">
            {signal.signal === 'bullish' ? '↗️ Haussier' : signal.signal === 'bearish' ? '↘️ Baissier' : '➡️ Neutre'}
          </div>
          <div class="probability">{signal.win_probability}%</div>
        </div>
      {/each}
    </div>
  </section>
  
  <section class="quick-actions">
    <h2>Actions Rapides</h2>
    <div class="actions-grid">
      <a href="/scanner" class="action-card">
        <span class="action-icon">🔍</span>
        <span class="action-label">Scanner V1-V5</span>
      </a>
      <a href="/markets" class="action-card">
        <span class="action-icon">📊</span>
        <span class="action-label">Vue Marchés</span>
      </a>
      <a href="/orderflow" class="action-card">
        <span class="action-icon">💹</span>
        <span class="action-label">Order Flow</span>
      </a>
      <a href="/risk" class="action-card">
        <span class="action-icon">🛡️</span>
        <span class="action-label">Gestion Risque</span>
      </a>
    </div>
  </section>
</div>

<style>
  .dashboard {
    max-width: 1400px;
    margin: 0 auto;
  }
  
  .header {
    margin-bottom: 2rem;
  }
  
  .header h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    color: #ff6b35;
    text-shadow: 0 0 30px rgba(255, 107, 53, 0.4);
  }
  
  .subtitle {
    color: #95a5a6;
    margin: 0;
  }
  
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }
  
  .stat-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
  }
  
  .stat-card.positive {
    border-color: rgba(38, 166, 154, 0.4);
  }
  
  .stat-label {
    color: #95a5a6;
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }
  
  .stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: #ff6b35;
  }
  
  .stat-card.positive .stat-value {
    color: #26a69a;
  }
  
  .stat-percent {
    font-size: 0.875rem;
    margin-left: 0.5rem;
  }
  
  .top-signals {
    margin-bottom: 2rem;
  }
  
  .top-signals h2 {
    color: #ff6b35;
    margin-bottom: 1rem;
  }
  
  .signals-table {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    overflow: hidden;
  }
  
  .table-header,
  .table-row {
    display: grid;
    grid-template-columns: 1fr 2fr 0.8fr 1.2fr 1fr;
    gap: 1rem;
    padding: 1rem 1.5rem;
    align-items: center;
  }
  
  .table-header {
    background: rgba(255, 107, 53, 0.1);
    font-weight: 600;
    color: #ff6b35;
  }
  
  .table-row {
    border-top: 1px solid rgba(255, 107, 53, 0.1);
  }
  
  .table-row:hover {
    background: rgba(255, 107, 53, 0.05);
  }
  
  .symbol {
    font-weight: 700;
    font-size: 1.1rem;
  }
  
  .score {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .score-bar {
    height: 8px;
    border-radius: 4px;
    transition: width 0.3s;
  }
  
  .score-text {
    font-weight: 600;
  }
  
  .grade {
    font-size: 1.5rem;
    font-weight: 700;
  }
  
  .probability {
    font-weight: 600;
  }
  
  .quick-actions h2 {
    color: #ff6b35;
    margin-bottom: 1rem;
  }
  
  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }
  
  .action-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-decoration: none;
    color: #d1d4dc;
    transition: all 0.2s;
  }
  
  .action-card:hover {
    border-color: #ff6b35;
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(255, 107, 53, 0.2);
  }
  
  .action-icon {
    font-size: 2rem;
  }
  
  .action-label {
    font-weight: 600;
  }
</style>
