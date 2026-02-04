<script>
  import { scannerResults } from '$lib/stores/scanner';
  import { focusSymbol } from '$lib/stores/focus';

  const HOT = 90;
  const CRITICAL = 95;
</script>

<div class="panel">
  <h3 style="color:var(--accent);font-weight:900;">SCANNER LIVE</h3>

  <table class="scan-table">
    <thead>
      <tr>
        <th>Symbol</th>
        <th>Setup</th>
        <th>Score</th>
      </tr>
    </thead>

    <tbody>
      {#each $scannerResults as s}
        <tr
          class:critical={s.score >= CRITICAL}
          on:click={() => focusSymbol.set(s.symbol)}
          style="cursor:pointer;"
        >
          <td style="font-weight:900;color:var(--accent-soft);">{s.symbol}</td>
          <td class="muted">{s.setup}</td>

          <td>
            {#if s.score >= HOT}
              <span class="score-hot">{s.score}</span>
            {:else}
              {s.score}
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
.score-hot{
  color:var(--accent);
  font-weight:900;
  padding:0.15rem 0.45rem;
  border-radius:8px;
  animation:flash 1s ease-in-out infinite;
  display:inline-block;
}

tr.critical{
  box-shadow: inset 0 0 0 9999px rgba(255,179,71,.10);
}

@keyframes flash{
  0%   { background: rgba(255,179,71,.12); }
  50%  { background: rgba(255,179,71,.42); }
  100% { background: rgba(255,179,71,.12); }
}
</style>
