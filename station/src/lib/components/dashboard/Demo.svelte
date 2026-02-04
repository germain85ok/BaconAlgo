<!-- Example Usage of Dashboard Components -->
<script lang="ts">
  import { SignalCard, ScoreGauge, GradeBadge, FilterSidebar } from '$lib/components/dashboard';

  // Example signal data
  const exampleSignal = {
    ticker: 'AAPL',
    direction: 'LONG' as const,
    score: 85,
    grade: 'A' as const,
    entry: 175.50,
    stop_loss: 172.00,
    take_profit_1: 178.00,
    take_profit_2: 180.50,
    take_profit_3: 183.00,
    risk_reward_ratio: 2.5,
    smc_tags: ['FVG', 'OB', 'BOS'],
    horizon: 'Day' as const,
    timestamp: new Date().toISOString()
  };

  // Example filters
  let filters = $state({
    horizon: 'all' as const,
    direction: 'all' as const,
    minScore: 0,
    minRR: 0,
    assetTypes: {
      stocks: true,
      crypto: true,
      forex: true,
      futures: true
    },
    smcFilters: {
      nearFVG: false,
      nearOB: false,
      bosConfirmed: false,
      choch: false,
      liquidity: false,
      imbalance: false
    }
  });

  const handleFilterChange = (newFilters: typeof filters) => {
    console.log('Filters changed:', newFilters);
    // Apply filters to signal list
  };

  const handleAddToWatchlist = (signal: typeof exampleSignal) => {
    console.log('Add to watchlist:', signal.ticker);
  };

  const handleSetAlert = (signal: typeof exampleSignal) => {
    console.log('Set alert:', signal.ticker);
  };
</script>

<div class="demo-container">
  <h1>BaconAlgo Dashboard Components Demo</h1>

  <section class="demo-section">
    <h2>Score Gauge</h2>
    <div class="gauge-examples">
      <div class="gauge-item">
        <ScoreGauge score={25} size="sm" />
        <p>Low (25)</p>
      </div>
      <div class="gauge-item">
        <ScoreGauge score={50} size="md" />
        <p>Medium (50)</p>
      </div>
      <div class="gauge-item">
        <ScoreGauge score={85} size="lg" />
        <p>High (85)</p>
      </div>
    </div>
  </section>

  <section class="demo-section">
    <h2>Grade Badges</h2>
    <div class="badge-examples">
      <GradeBadge grade="S" />
      <GradeBadge grade="A" />
      <GradeBadge grade="B" />
      <GradeBadge grade="C" />
    </div>
  </section>

  <section class="demo-section">
    <h2>Signal Card</h2>
    <div class="card-example">
      <SignalCard
        signal={exampleSignal}
        onAddToWatchlist={handleAddToWatchlist}
        onSetAlert={handleSetAlert}
      />
    </div>
  </section>

  <section class="demo-section">
    <h2>Filter Sidebar</h2>
    <div class="filter-example">
      <FilterSidebar bind:filters onFilterChange={handleFilterChange} />
    </div>
  </section>
</div>

<style>
  .demo-container {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
    background: #0f172a;
    min-height: 100vh;
  }

  h1 {
    color: #ff6b35;
    margin-bottom: 2rem;
    text-align: center;
  }

  h2 {
    color: #fbbf24;
    margin-bottom: 1rem;
  }

  .demo-section {
    margin-bottom: 3rem;
    padding: 2rem;
    background: rgba(17, 24, 39, 0.5);
    border-radius: 1rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .gauge-examples {
    display: flex;
    gap: 2rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .gauge-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .gauge-item p {
    color: #e5e7eb;
    font-size: 0.875rem;
  }

  .badge-examples {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .card-example {
    max-width: 400px;
    margin: 0 auto;
  }

  .filter-example {
    max-width: 350px;
  }

  @media (max-width: 768px) {
    .demo-container {
      padding: 1rem;
    }

    .demo-section {
      padding: 1rem;
    }
  }
</style>
