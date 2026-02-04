<script>
  import { onMount } from 'svelte';
  import { filteredSignals, stats, connectSignalStream } from '$lib/stores/signals';
  import SignalCard from '$lib/components/SignalCard.svelte';
  import StatsGrid from '$lib/components/StatsGrid.svelte';
  import Filters from '$lib/components/Filters.svelte';
  import '../lib/styles/bacon-2030.css';

  let cleanup;

  onMount(() => {
    cleanup = connectSignalStream('http://localhost:3000/signals/live');
    return () => cleanup && cleanup();
  });
</script>

<svelte:head>
  <title>🥓 BaconAlgo 2030 - Ultimate Trading Platform</title>
</svelte:head>

<div class="dashboard">
  <header class="header">
    <h1 class="gradient-text">🥓 BaconAlgo 2030</h1>
    <p class="tagline">Ultra-Modern Trading Platform with Smart Money Concepts</p>
  </header>

  <div class="main-container">
    <aside class="sidebar">
      <Filters />
    </aside>

    <main class="main-content">
      <StatsGrid stats={$stats} />

      <div class="signals-section">
        <h2>Live Signals</h2>
        {#if $filteredSignals.length === 0}
          <div class="empty-state glass-card">
            <p>🔍 No signals match your filters...</p>
            <p class="hint">Try adjusting your filters or wait for new signals</p>
          </div>
        {:else}
          <div class="signals-grid">
            {#each $filteredSignals as signal, i (signal.id)}
              <SignalCard {signal} rank={i + 1} />
            {/each}
          </div>
        {/if}
      </div>
    </main>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }

  .dashboard {
    min-height: 100vh;
    padding: 2rem;
  }

  .header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .header h1 {
    font-size: 3rem;
    font-weight: 900;
    margin: 0;
  }

  .tagline {
    font-size: 1.125rem;
    color: rgba(255, 255, 255, 0.6);
    margin-top: 0.5rem;
  }

  .main-container {
    display: grid;
    grid-template-columns: 300px 1fr;
    gap: 2rem;
  }

  .sidebar {
    position: sticky;
    top: 2rem;
    height: fit-content;
  }

  .main-content {
    min-width: 0;
  }

  .signals-section {
    margin-top: 2rem;
  }

  .signals-section h2 {
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
  }

  .empty-state p {
    font-size: 1.125rem;
    margin-bottom: 0.5rem;
  }

  .empty-state .hint {
    font-size: 0.875rem;
    color: rgba(255, 255, 255, 0.5);
  }

  @media (max-width: 1024px) {
    .main-container {
      grid-template-columns: 1fr;
    }

    .sidebar {
      position: static;
    }
  }
</style>
