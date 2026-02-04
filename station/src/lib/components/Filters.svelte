<script lang="ts">
  import { filters } from '../stores/signals';

  let minScore = 0;
  let minWhale = 0;
  let minConfluence = 0;
  let minPower = 0;
  let setupFilters = {
    fvg: false,
    ob: false,
    goldenPocket: false,
    bos: false,
    choch: false,
    npoc: false,
    avwap: false,
  };

  $: filters.set({
    minScore,
    minWhale,
    minConfluence,
    minPower,
    setups: setupFilters,
  });
</script>

<div class="filters glass-card">
  <h3>Filters</h3>
  
  <div class="filter-section">
    <label>
      <span>Min Score: {minScore}</span>
      <input type="range" bind:value={minScore} min="0" max="100" />
    </label>
  </div>

  <div class="filter-section">
    <label>
      <span>Min Whale: {minWhale}</span>
      <input type="range" bind:value={minWhale} min="0" max="100" />
    </label>
  </div>

  <div class="filter-section">
    <label>
      <span>Min Confluence: {minConfluence}</span>
      <input type="range" bind:value={minConfluence} min="0" max="10" />
    </label>
  </div>

  <div class="filter-section">
    <label>
      <span>Min Power: {minPower}</span>
      <input type="range" bind:value={minPower} min="0" max="500" step="10" />
    </label>
  </div>

  <div class="filter-section">
    <h4>Setup Filters</h4>
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.fvg} />
        <span>FVG</span>
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.ob} />
        <span>Order Blocks</span>
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.goldenPocket} />
        <span>Golden Pocket</span>
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.bos} />
        <span>BOS</span>
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.choch} />
        <span>CHoCH</span>
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.npoc} />
        <span>NPOC</span>
      </label>
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={setupFilters.avwap} />
        <span>AVWAP</span>
      </label>
    </div>
  </div>
</div>

<style>
  .filters {
    padding: 1.5rem;
  }

  h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.25rem;
  }

  h4 {
    margin: 0 0 0.75rem 0;
    font-size: 1rem;
  }

  .filter-section {
    margin-bottom: 1.5rem;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label span {
    font-size: 0.875rem;
    color: rgba(255, 255, 255, 0.8);
  }

  input[type="range"] {
    width: 100%;
    height: 6px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
  }

  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    background: var(--primary);
    border-radius: 50%;
    cursor: pointer;
  }

  input[type="range"]::-moz-range-thumb {
    width: 16px;
    height: 16px;
    background: var(--primary);
    border-radius: 50%;
    cursor: pointer;
    border: none;
  }

  .checkbox-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .checkbox-label {
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  input[type="checkbox"] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }
</style>
