<script lang="ts">
  export let stats: {
    total: number;
    legendary: number;
    epic: number;
    rare: number;
    avgScore: number;
    whaleSignals: number;
    avgConfluence: number;
    avgPower: number;
  };
</script>

<div class="stats-grid">
  <div class="stat-card glass-card">
    <div class="stat-icon">📊</div>
    <div class="stat-value">{stats.total}</div>
    <div class="stat-label">Total Signals</div>
  </div>

  <div class="stat-card glass-card legendary">
    <div class="stat-icon">🏆</div>
    <div class="stat-value">{stats.legendary}</div>
    <div class="stat-label">Legendary</div>
  </div>

  <div class="stat-card glass-card epic">
    <div class="stat-icon">⚡</div>
    <div class="stat-value">{stats.epic}</div>
    <div class="stat-label">Epic</div>
  </div>

  <div class="stat-card glass-card rare">
    <div class="stat-icon">💎</div>
    <div class="stat-value">{stats.rare}</div>
    <div class="stat-label">Rare</div>
  </div>

  <div class="stat-card glass-card">
    <div class="stat-icon">📈</div>
    <div class="stat-value">{stats.avgScore.toFixed(1)}</div>
    <div class="stat-label">Avg Score</div>
  </div>

  <div class="stat-card glass-card">
    <div class="stat-icon">🐋</div>
    <div class="stat-value">{stats.whaleSignals}</div>
    <div class="stat-label">Whale Signals</div>
  </div>

  <div class="stat-card glass-card">
    <div class="stat-icon">🎯</div>
    <div class="stat-value">{stats.avgConfluence.toFixed(1)}</div>
    <div class="stat-label">Avg Confluence</div>
  </div>

  <div class="stat-card glass-card">
    <div class="stat-icon">⚡</div>
    <div class="stat-value">{stats.avgPower.toFixed(0)}</div>
    <div class="stat-label">Avg Power</div>
  </div>
</div>

<style>
  .stat-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 1.5rem;
    text-align: center;
    min-height: 120px;
  }

  .stat-icon {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  .stat-value {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }

  .stat-label {
    font-size: 0.875rem;
    color: rgba(255, 255, 255, 0.6);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
</style>
