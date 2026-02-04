<script lang="ts">
  import ScoreGauge from './ScoreGauge.svelte';
  import GradeBadge from './GradeBadge.svelte';

  interface Signal {
    ticker: string;
    direction: 'LONG' | 'SHORT';
    score: number;
    grade: 'S' | 'A' | 'B' | 'C';
    entry: number;
    stop_loss: number;
    take_profit_1: number;
    take_profit_2: number;
    take_profit_3: number;
    risk_reward_ratio: number;
    smc_tags: string[];
    horizon: 'Scalp' | 'Day' | 'Swing' | 'Long';
    timestamp?: string;
  }

  interface Props {
    signal: Signal;
    onAddToWatchlist?: (signal: Signal) => void;
    onSetAlert?: (signal: Signal) => void;
  }

  let { signal, onAddToWatchlist, onSetAlert }: Props = $props();

  const directionColor = $derived(
    signal.direction === 'LONG' ? 'buy' : 'sell'
  );

  const directionLabel = $derived(
    signal.direction === 'LONG' ? 'BUY' : 'SELL'
  );

  const formatPrice = (price: number): string => {
    return price.toLocaleString('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    });
  };

  const formatTimestamp = (timestamp?: string): string => {
    if (!timestamp) return 'Just now';
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    if (minutes < 1) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    const hours = Math.floor(minutes / 60);
    if (hours < 24) return `${hours}h ago`;
    return date.toLocaleDateString();
  };

  const handleAddToWatchlist = () => {
    onAddToWatchlist?.(signal);
  };

  const handleSetAlert = () => {
    onSetAlert?.(signal);
  };
</script>

<div class="signal-card">
  <div class="card-header">
    <div class="ticker-row">
      <h3 class="ticker">{signal.ticker}</h3>
      <div class="direction-badge {directionColor}">
        {directionLabel}
      </div>
    </div>
    <div class="meta-row">
      <GradeBadge grade={signal.grade} />
      <span class="horizon">{signal.horizon}</span>
    </div>
  </div>

  <div class="card-body">
    <div class="score-section">
      <ScoreGauge score={signal.score} size="md" />
    </div>

    <div class="price-levels">
      <div class="price-item entry">
        <span class="label">Entry</span>
        <span class="value">${formatPrice(signal.entry)}</span>
      </div>
      <div class="price-item stop-loss">
        <span class="label">Stop Loss</span>
        <span class="value">${formatPrice(signal.stop_loss)}</span>
      </div>
      <div class="price-item take-profit">
        <span class="label">TP1</span>
        <span class="value">${formatPrice(signal.take_profit_1)}</span>
      </div>
      <div class="price-item take-profit">
        <span class="label">TP2</span>
        <span class="value">${formatPrice(signal.take_profit_2)}</span>
      </div>
      <div class="price-item take-profit">
        <span class="label">TP3</span>
        <span class="value">${formatPrice(signal.take_profit_3)}</span>
      </div>
    </div>

    <div class="risk-reward">
      <span class="label">Risk:Reward</span>
      <span class="value">1:{signal.risk_reward_ratio.toFixed(2)}</span>
    </div>

    {#if signal.smc_tags && signal.smc_tags.length > 0}
      <div class="smc-tags">
        {#each signal.smc_tags as tag}
          <span class="tag">{tag}</span>
        {/each}
      </div>
    {/if}
  </div>

  <div class="card-footer">
    <span class="timestamp">{formatTimestamp(signal.timestamp)}</span>
    <div class="actions">
      <button class="action-btn watchlist" onclick={handleAddToWatchlist}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        Watchlist
      </button>
      <button class="action-btn alert" onclick={handleSetAlert}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
          <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
        </svg>
        Alert
      </button>
    </div>
  </div>
</div>

<style>
  .signal-card {
    background: rgba(17, 24, 39, 0.7);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 1rem;
    overflow: hidden;
    transition: all 0.3s ease;
  }

  .signal-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 20px 40px rgba(255, 107, 53, 0.2);
    border-color: rgba(255, 107, 53, 0.3);
  }

  .card-header {
    padding: 1.25rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .ticker-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .ticker {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f9fafb;
    margin: 0;
  }

  .direction-badge {
    padding: 0.375rem 0.875rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.05em;
  }

  .direction-badge.buy {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: #fff;
    box-shadow: 0 0 20px rgba(16, 185, 129, 0.4);
  }

  .direction-badge.sell {
    background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
    color: #fff;
    box-shadow: 0 0 20px rgba(239, 68, 68, 0.4);
  }

  .meta-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .horizon {
    padding: 0.25rem 0.625rem;
    background: rgba(255, 107, 53, 0.1);
    border: 1px solid rgba(255, 107, 53, 0.3);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 600;
    color: #ff6b35;
  }

  .card-body {
    padding: 1.25rem;
  }

  .score-section {
    display: flex;
    justify-content: center;
    margin-bottom: 1.5rem;
  }

  .price-levels {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .price-item {
    display: flex;
    justify-content: space-between;
    padding: 0.625rem;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .price-item.entry {
    border-color: rgba(59, 130, 246, 0.3);
    background: rgba(59, 130, 246, 0.05);
  }

  .price-item.stop-loss {
    border-color: rgba(239, 68, 68, 0.3);
    background: rgba(239, 68, 68, 0.05);
  }

  .price-item.take-profit {
    border-color: rgba(16, 185, 129, 0.3);
    background: rgba(16, 185, 129, 0.05);
  }

  .price-item .label {
    font-size: 0.75rem;
    color: #9ca3af;
    font-weight: 500;
  }

  .price-item .value {
    font-size: 0.875rem;
    color: #f9fafb;
    font-weight: 600;
  }

  .risk-reward {
    display: flex;
    justify-content: space-between;
    padding: 0.75rem;
    background: linear-gradient(135deg, rgba(255, 107, 53, 0.1) 0%, rgba(251, 191, 36, 0.1) 100%);
    border: 1px solid rgba(255, 107, 53, 0.3);
    border-radius: 0.5rem;
    margin-bottom: 1rem;
  }

  .risk-reward .label {
    font-size: 0.875rem;
    color: #ff6b35;
    font-weight: 600;
  }

  .risk-reward .value {
    font-size: 1rem;
    color: #fbbf24;
    font-weight: 700;
  }

  .smc-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag {
    padding: 0.25rem 0.625rem;
    background: rgba(168, 85, 247, 0.1);
    border: 1px solid rgba(168, 85, 247, 0.3);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 600;
    color: #a855f7;
  }

  .card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(0, 0, 0, 0.2);
  }

  .timestamp {
    font-size: 0.75rem;
    color: #9ca3af;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.875rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 0.5rem;
    background: rgba(255, 255, 255, 0.05);
    color: #e5e7eb;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .action-btn:hover {
    background: rgba(255, 107, 53, 0.1);
    border-color: rgba(255, 107, 53, 0.3);
    color: #ff6b35;
    transform: translateY(-2px);
  }

  .action-btn.watchlist:hover {
    background: linear-gradient(135deg, rgba(255, 107, 53, 0.2) 0%, rgba(251, 191, 36, 0.2) 100%);
  }

  /* Responsive */
  @media (max-width: 640px) {
    .card-header,
    .card-body,
    .card-footer {
      padding: 1rem;
    }

    .ticker {
      font-size: 1.25rem;
    }

    .price-levels {
      grid-template-columns: 1fr;
    }

    .card-footer {
      flex-direction: column;
      gap: 0.75rem;
      align-items: stretch;
    }

    .actions {
      width: 100%;
      justify-content: stretch;
    }

    .action-btn {
      flex: 1;
      justify-content: center;
    }
  }

  @media (min-width: 641px) and (max-width: 1024px) {
    .price-levels {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
