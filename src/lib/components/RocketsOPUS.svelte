<script>
  import { rockets } from '$lib/stores/rockets';
</script>

<div class="panel">
  <h3 style="color:var(--accent);font-weight:900;">ROQUETTES</h3>

  <div class="stack">
    {#each $rockets as r}
      <div class="rocket glass-card">
        <div class="top">
          <div class="sym">{r.symbol}</div>
          <div class="state muted">{r.state}</div>
        </div>
        <div class="score">score: {r.score}</div>
      </div>
    {/each}
  </div>
</div>

<style>
.stack{
  display:flex;
  flex-direction:column;
  gap:0.8rem;
}

.rocket{
  animation:pulse 2s ease-in-out infinite;
}

.top{
  display:flex;
  justify-content:space-between;
  align-items:baseline;
  gap:0.75rem;
}

.sym{
  font-weight:900;
  color:var(--accent);
  letter-spacing:0.06em;
}

.score{
  margin-top:0.35rem;
  font-weight:800;
  color:var(--accent-soft);
}

@keyframes pulse{
  0% { box-shadow: 0 0 10px rgba(255,179,71,.18); transform:translateY(0); }
  50% { box-shadow: 0 0 24px rgba(255,179,71,.55); transform:translateY(-1px); }
  100% { box-shadow: 0 0 10px rgba(255,179,71,.18); transform:translateY(0); }
}
</style>
