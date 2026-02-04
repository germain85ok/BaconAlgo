<script lang="ts">
  import type { Signal } from '../stores/signals';
  
  export let signal: Signal;
  export let rank: number = 0;

  function getGradeClass(grade: string): string {
    return grade.toLowerCase();
  }

  function formatPrice(price: number): string {
    return price.toFixed(2);
  }

  function formatPercent(value: number): string {
    return `${value.toFixed(1)}%`;
  }

  function getSetupTags(): string[] {
    const tags: string[] = [];
    if (signal.near_fvg) tags.push('FVG');
    if (signal.near_ob) tags.push('OB');
    if (signal.near_golden_pocket) tags.push('Golden Pocket');
    if (signal.bos_confirmed) tags.push('BOS');
    if (signal.choch_detected) tags.push('CHoCH');
    if (signal.near_npoc) tags.push('NPOC');
    if (signal.near_avwap) tags.push('AVWAP');
    return tags;
  }

  $: gradeClass = getGradeClass(signal.grade);
  $: setupTags = getSetupTags();
</script>

<div class="signal-card glass-card {gradeClass} fade-in">
  <div class="card-header">
    {#if rank > 0}
      <span class="rank">#{rank}</span>
    {/if}
    <span class="badge badge-{gradeClass}">{signal.grade}</span>
    <h3 class="symbol">{signal.symbol}</h3>
    <span class="horizon">{signal.horizon}</span>
    <span class="direction {signal.direction.toLowerCase()}">{signal.direction}</span>
  </div>

  <div class="score-display">
    <div class="score-circle">
      <svg viewBox="0 0 100 100">
        <circle cx="50" cy="50" r="45" class="score-bg" />
        <circle cx="50" cy="50" r="45" class="score-fill" 
          style="stroke-dasharray: {signal.score * 2.827}, 282.7" />
      </svg>
      <div class="score-value">{signal.score}</div>
    </div>
  </div>

  <div class="metrics">
    <div class="metric">
      <span class="metric-label">Entry</span>
      <span class="metric-value">${formatPrice(signal.entry)}</span>
    </div>
    <div class="metric">
      <span class="metric-label">Stop</span>
      <span class="metric-value danger">${formatPrice(signal.stop_loss)}</span>
    </div>
    <div class="metric">
      <span class="metric-label">Target</span>
      <span class="metric-value success">
        ${signal.targets.length > 0 ? formatPrice(signal.targets[0]) : 'N/A'}
      </span>
    </div>
    <div class="metric">
      <span class="metric-label">R:R</span>
      <span class="metric-value">{signal.risk_reward.toFixed(2)}</span>
    </div>
  </div>

  <div class="extended-metrics">
    <div class="metric-row">
      <span class="label">Confluence</span>
      <span class="value">{signal.confluence_count}</span>
    </div>
    <div class="metric-row">
      <span class="label">Power</span>
      <span class="value">{signal.power_score}</span>
    </div>
    {#if signal.whale_score > 0}
      <div class="metric-row">
        <span class="label">🐋 Whale</span>
        <span class="value">{signal.whale_score}</span>
      </div>
    {/if}
  </div>

  {#if setupTags.length > 0}
    <div class="setup-badges">
      {#each setupTags as tag}
        <span class="setup-badge">{tag}</span>
      {/each}
    </div>
  {/if}

  {#if signal.wave_phase}
    <div class="wave-phase">
      <span class="label">Elliott Wave</span>
      <span class="value">{signal.wave_phase}</span>
    </div>
  {/if}
</div>

<style>
  .signal-card {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-height: 300px;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .rank {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--gold);
  }

  .symbol {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    flex: 1;
  }

  .horizon {
    padding: 0.25rem 0.5rem;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    font-size: 0.75rem;
  }

  .direction {
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .direction.long {
    background: rgba(0, 217, 255, 0.2);
    color: var(--success);
  }

  .direction.short {
    background: rgba(248, 81, 73, 0.2);
    color: var(--danger);
  }

  .score-display {
    display: flex;
    justify-content: center;
    padding: 1rem 0;
  }

  .score-circle {
    position: relative;
    width: 120px;
    height: 120px;
  }

  .score-circle svg {
    transform: rotate(-90deg);
    width: 100%;
    height: 100%;
  }

  .score-bg {
    fill: none;
    stroke: rgba(255, 255, 255, 0.1);
    stroke-width: 8;
  }

  .score-fill {
    fill: none;
    stroke: var(--primary);
    stroke-width: 8;
    stroke-linecap: round;
    transition: stroke-dasharray 0.5s ease;
  }

  .legendary .score-fill {
    stroke: var(--gold);
  }

  .epic .score-fill {
    stroke: var(--purple);
  }

  .rare .score-fill {
    stroke: var(--success);
  }

  .score-value {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 2rem;
    font-weight: 700;
  }

  .metrics {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .metric {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .metric-label {
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.6);
    text-transform: uppercase;
  }

  .metric-value {
    font-size: 1.125rem;
    font-weight: 600;
  }

  .metric-value.success {
    color: var(--success);
  }

  .metric-value.danger {
    color: var(--danger);
  }

  .extended-metrics {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .metric-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.875rem;
  }

  .metric-row .label {
    color: rgba(255, 255, 255, 0.6);
  }

  .metric-row .value {
    font-weight: 600;
  }

  .setup-badges {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .setup-badge {
    padding: 0.25rem 0.5rem;
    background: rgba(255, 107, 53, 0.2);
    border: 1px solid rgba(255, 107, 53, 0.4);
    border-radius: 6px;
    font-size: 0.625rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--primary);
  }

  .wave-phase {
    display: flex;
    justify-content: space-between;
    padding: 0.75rem;
    background: rgba(168, 85, 247, 0.1);
    border-radius: 8px;
    font-size: 0.875rem;
  }

  .wave-phase .label {
    color: rgba(255, 255, 255, 0.6);
  }

  .wave-phase .value {
    font-weight: 600;
    color: var(--purple);
  }
</style>
