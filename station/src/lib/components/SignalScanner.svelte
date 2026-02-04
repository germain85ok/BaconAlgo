<script lang="ts">
  import { onMount } from 'svelte';

  interface SignalData {
    symbol: string;
    ready: boolean;
    score: number;
    type?: string;
  }

  let signals: SignalData[] = [];

  onMount(() => {
    const es = new EventSource('/signals/live');

    es.onmessage = (e) => {
      const data = JSON.parse(e.data);
      if (data.type !== 'signal') return;

      signals = [
        data,
        ...signals.filter(s => s.symbol !== data.symbol)
      ].slice(0, 10);
    };
  });
</script>

<style>
  .scanner {
    font-family: monospace;
    background: #0b0b0b;
    color: #fff;
    padding: 12px;
    border-radius: 8px;
  }
  .row {
    display: flex;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid #222;
  }
  .ready { color: #00ff88; }
  .cooldown { color: #ff5555; }
</style>

<div class="scanner">
  {#each signals as s}
    <div class="row">
      <span>{s.symbol}</span>
      <span class={s.ready ? 'ready' : 'cooldown'}>
        {s.ready ? 'READY' : 'WAIT'}
      </span>
      <span>{s.score}</span>
    </div>
  {/each}
</div>
