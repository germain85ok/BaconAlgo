<script>
  import { sentimentRaw } from '$lib/stores/sentiment';
</script>

<div class="panel">
  <h3 style="color:var(--accent);font-weight:900;">SENTIMENT GLOBAL</h3>

  <div class="grid">
    {#each Object.values($sentimentRaw) as s}
      <div class="glass-card">
        <div class="row">
          <div class="k">{s.key}</div>
          <div class="muted">{s.value}</div>
        </div>

        <div class="chg {s.change >= 0 ? 'pos' : 'neg'}">
          {s.change}%
        </div>

        <div class="bar">
          <div class="fill" style="width:{s.confidence}%"></div>
        </div>

        <div class="muted" style="font-size:0.8rem;margin-top:0.4rem;">
          confidence: {s.confidence}%
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
.grid{
  display:grid;
  grid-template-columns:repeat(3,1fr);
  gap:1rem;
}

.row{
  display:flex;
  justify-content:space-between;
  align-items:baseline;
  gap:0.8rem;
}

.k{
  font-weight:900;
  color:var(--accent-soft);
}

.chg{
  margin-top:0.35rem;
  font-weight:900;
}

.pos{ color:var(--accent); }
.neg{ color:var(--negative); }

.bar{
  height:6px;
  background:rgba(255,255,255,.06);
  border-radius:999px;
  overflow:hidden;
  margin-top:0.5rem;
  border:1px solid rgba(255,179,71,.15);
}

.fill{
  height:100%;
  background:linear-gradient(90deg,var(--accent),var(--accent-soft));
  transition:width 700ms ease;
}
</style>
