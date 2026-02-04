<script>
  import { streamStatus } from '$lib/stores/stream';
</script>

<div class="panel header">
  <div class="brand">
    <div class="title">BaconAlgo OPUS</div>
    <div class="subtitle muted">ULTIMATE STATION</div>
  </div>

  <div class="status">
    <span class="dot {$streamStatus === 'live' ? 'live' : 'down'}"></span>
    <span class="label">
      {$streamStatus === 'live' ? 'STREAM LIVE' : 'STREAM DOWN'}
    </span>
  </div>
</div>

<style>
.header{
  display:flex;
  justify-content:space-between;
  align-items:center;
  gap:1rem;
}

.title{
  font-weight:900;
  font-size:1.2rem;
  color:var(--accent);
  letter-spacing:0.02em;
}

.subtitle{
  font-size:0.72rem;
  letter-spacing:0.18em;
  margin-top:0.2rem;
}

.status{
  display:flex;
  align-items:center;
  gap:0.6rem;
  padding:0.5rem 0.85rem;
  border-radius:999px;
  border:1px solid var(--border);
  background:rgba(255,179,71,.06);
}

.dot{
  width:10px;
  height:10px;
  border-radius:50%;
  display:inline-block;
}

.dot.live{
  background:var(--accent);
  box-shadow:0 0 18px rgba(255,179,71,.65);
}

.dot.down{
  background:var(--negative);
  box-shadow:0 0 18px rgba(255,77,77,.45);
}

.label{
  font-weight:900;
  letter-spacing:0.08em;
  color:var(--accent-soft);
  font-size:0.85rem;
}
</style>
