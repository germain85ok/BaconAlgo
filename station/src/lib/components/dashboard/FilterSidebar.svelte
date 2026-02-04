<script lang="ts">
  interface Filters {
    horizon: 'all' | 'Scalp' | 'Day' | 'Swing' | 'Long';
    direction: 'all' | 'LONG' | 'SHORT';
    minScore: number;
    minRR: number;
    assetTypes: {
      stocks: boolean;
      crypto: boolean;
      forex: boolean;
      futures: boolean;
    };
    smcFilters: {
      nearFVG: boolean;
      nearOB: boolean;
      bosConfirmed: boolean;
      choch: boolean;
      liquidity: boolean;
      imbalance: boolean;
    };
  }

  interface Props {
    filters: Filters;
    onFilterChange?: (filters: Filters) => void;
  }

  let { filters = $bindable(), onFilterChange }: Props = $props();

  const defaultFilters: Filters = {
    horizon: 'all',
    direction: 'all',
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
  };

  const resetFilters = () => {
    filters = { ...defaultFilters };
    onFilterChange?.(filters);
  };

  const applyFilters = () => {
    onFilterChange?.(filters);
  };

  const handleHorizonChange = (value: Filters['horizon']) => {
    filters.horizon = value;
  };

  const handleDirectionChange = (value: Filters['direction']) => {
    filters.direction = value;
  };
</script>

<aside class="filter-sidebar">
  <div class="sidebar-header">
    <h3>Filters</h3>
    <button class="reset-btn" onclick={resetFilters}>Reset</button>
  </div>

  <div class="filters-content">
    <!-- Horizon Section -->
    <section class="filter-section">
      <h4 class="section-title">Horizon</h4>
      <div class="radio-group">
        <label class="radio-label">
          <input
            type="radio"
            name="horizon"
            value="all"
            checked={filters.horizon === 'all'}
            onchange={() => handleHorizonChange('all')}
          />
          <span>All</span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            name="horizon"
            value="Scalp"
            checked={filters.horizon === 'Scalp'}
            onchange={() => handleHorizonChange('Scalp')}
          />
          <span>Scalp</span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            name="horizon"
            value="Day"
            checked={filters.horizon === 'Day'}
            onchange={() => handleHorizonChange('Day')}
          />
          <span>Day</span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            name="horizon"
            value="Swing"
            checked={filters.horizon === 'Swing'}
            onchange={() => handleHorizonChange('Swing')}
          />
          <span>Swing</span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            name="horizon"
            value="Long"
            checked={filters.horizon === 'Long'}
            onchange={() => handleHorizonChange('Long')}
          />
          <span>Long</span>
        </label>
      </div>
    </section>

    <!-- Direction Section -->
    <section class="filter-section">
      <h4 class="section-title">Direction</h4>
      <div class="radio-group">
        <label class="radio-label">
          <input
            type="radio"
            name="direction"
            value="all"
            checked={filters.direction === 'all'}
            onchange={() => handleDirectionChange('all')}
          />
          <span>Both</span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            name="direction"
            value="LONG"
            checked={filters.direction === 'LONG'}
            onchange={() => handleDirectionChange('LONG')}
          />
          <span>Long</span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            name="direction"
            value="SHORT"
            checked={filters.direction === 'SHORT'}
            onchange={() => handleDirectionChange('SHORT')}
          />
          <span>Short</span>
        </label>
      </div>
    </section>

    <!-- Min Score Section -->
    <section class="filter-section">
      <h4 class="section-title">
        Min Score
        <span class="value-indicator">{filters.minScore}</span>
      </h4>
      <input
        type="range"
        min="0"
        max="100"
        step="5"
        bind:value={filters.minScore}
        class="slider"
      />
    </section>

    <!-- Min R:R Section -->
    <section class="filter-section">
      <h4 class="section-title">
        Min R:R
        <span class="value-indicator">{filters.minRR.toFixed(1)}</span>
      </h4>
      <input
        type="range"
        min="0"
        max="10"
        step="0.5"
        bind:value={filters.minRR}
        class="slider"
      />
    </section>

    <!-- Asset Type Section -->
    <section class="filter-section">
      <h4 class="section-title">Asset Type</h4>
      <div class="checkbox-group">
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.assetTypes.stocks} />
          <span>Stocks</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.assetTypes.crypto} />
          <span>Crypto</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.assetTypes.forex} />
          <span>Forex</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.assetTypes.futures} />
          <span>Futures</span>
        </label>
      </div>
    </section>

    <!-- SMC Filters Section -->
    <section class="filter-section">
      <h4 class="section-title">SMC Filters</h4>
      <div class="checkbox-group">
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.smcFilters.nearFVG} />
          <span>Near FVG</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.smcFilters.nearOB} />
          <span>Near OB</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.smcFilters.bosConfirmed} />
          <span>BOS Confirmed</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.smcFilters.choch} />
          <span>CHoCH</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.smcFilters.liquidity} />
          <span>Liquidity</span>
        </label>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={filters.smcFilters.imbalance} />
          <span>Imbalance</span>
        </label>
      </div>
    </section>
  </div>

  <div class="sidebar-footer">
    <button class="apply-btn" onclick={applyFilters}>Apply Filters</button>
  </div>
</aside>

<style>
  .filter-sidebar {
    position: sticky;
    top: 1rem;
    height: fit-content;
    max-height: calc(100vh - 2rem);
    background: rgba(17, 24, 39, 0.8);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 1rem;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .sidebar-header h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 700;
    color: #f9fafb;
  }

  .reset-btn {
    padding: 0.375rem 0.75rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 0.375rem;
    color: #9ca3af;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .reset-btn:hover {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }

  .filters-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.25rem;
  }

  .filters-content::-webkit-scrollbar {
    width: 6px;
  }

  .filters-content::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
  }

  .filters-content::-webkit-scrollbar-thumb {
    background: rgba(255, 107, 53, 0.3);
    border-radius: 3px;
  }

  .filters-content::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 107, 53, 0.5);
  }

  .filter-section {
    margin-bottom: 1.75rem;
  }

  .filter-section:last-child {
    margin-bottom: 0;
  }

  .section-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin: 0 0 0.75rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    color: #ff6b35;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .value-indicator {
    color: #fbbf24;
    font-weight: 700;
  }

  .radio-group,
  .checkbox-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .radio-label,
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.625rem;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .radio-label:hover,
  .checkbox-label:hover {
    background: rgba(255, 107, 53, 0.05);
    border-color: rgba(255, 107, 53, 0.2);
  }

  .radio-label input[type="radio"],
  .checkbox-label input[type="checkbox"] {
    width: 1rem;
    height: 1rem;
    cursor: pointer;
    accent-color: #ff6b35;
  }

  .radio-label span,
  .checkbox-label span {
    font-size: 0.875rem;
    color: #e5e7eb;
    font-weight: 500;
  }

  .slider {
    width: 100%;
    height: 6px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
    cursor: pointer;
    border-radius: 50%;
    box-shadow: 0 0 10px rgba(255, 107, 53, 0.5);
    transition: all 0.2s ease;
  }

  .slider::-webkit-slider-thumb:hover {
    transform: scale(1.2);
    box-shadow: 0 0 15px rgba(255, 107, 53, 0.8);
  }

  .slider::-moz-range-thumb {
    width: 18px;
    height: 18px;
    background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
    cursor: pointer;
    border-radius: 50%;
    border: none;
    box-shadow: 0 0 10px rgba(255, 107, 53, 0.5);
    transition: all 0.2s ease;
  }

  .slider::-moz-range-thumb:hover {
    transform: scale(1.2);
    box-shadow: 0 0 15px rgba(255, 107, 53, 0.8);
  }

  .sidebar-footer {
    padding: 1.25rem;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(0, 0, 0, 0.2);
  }

  .apply-btn {
    width: 100%;
    padding: 0.875rem;
    background: linear-gradient(135deg, #ff6b35 0%, #fbbf24 100%);
    border: none;
    border-radius: 0.5rem;
    color: #1f2937;
    font-size: 0.875rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 4px 15px rgba(255, 107, 53, 0.3);
  }

  .apply-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(255, 107, 53, 0.5);
  }

  .apply-btn:active {
    transform: translateY(0);
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .filter-sidebar {
      position: static;
      max-height: none;
    }
  }

  @media (max-width: 640px) {
    .sidebar-header,
    .filters-content,
    .sidebar-footer {
      padding: 1rem;
    }

    .filter-section {
      margin-bottom: 1.5rem;
    }
  }
</style>
