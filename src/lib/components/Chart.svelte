<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart } from 'lightweight-charts';
  import { liveCandles } from '$lib/stores/live';

  export let symbol;

  let container;
  let chart;
  let series;
  let unsubscribe;

  onMount(() => {
    chart = createChart(container, {
      layout: {
        background: { color: '#030305' },
        textColor: '#ffb347'
      },
      grid: {
        vertLines: { color: 'rgba(255,179,71,0.1)' },
        horzLines: { color: 'rgba(255,179,71,0.1)' }
      },
      timeScale: { timeVisible: true }
    });

    series = chart.addCandlestickSeries({
      upColor: '#ffb347',
      downColor: '#ff4d4d',
      borderUpColor: '#ffb347',
      borderDownColor: '#ff4d4d',
      wickUpColor: '#ffb347',
      wickDownColor: '#ff4d4d'
    });

    const source = new EventSource('http://localhost:3001/stream');

    source.onmessage = (event) => {
      const candle = JSON.parse(event.data);
      series.update(candle);
    };

    unsubscribe = () => source.close();
  });

  onDestroy(() => {
    unsubscribe?.();
    chart?.remove();
  });
</script>

<div class="panel" style="height:100%;min-height:360px;">
  <div style="font-weight:900;color:var(--accent);margin-bottom:0.5rem;">
    📈 {symbol} — LIVE
  </div>
  <div bind:this={container} style="height:300px;width:100%;"></div>
</div>
