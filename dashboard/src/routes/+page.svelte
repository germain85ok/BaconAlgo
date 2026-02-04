<script lang="ts">
  import { onMount } from "svelte";
  import { createChart } from "lightweight-charts";

  let container: HTMLDivElement;

  const API = "http://127.0.0.1:3000";
  const symbol = "AAPL";

  type Bar = {
    symbol: string;
    timeframe: string;
    ts: string;
    open: number;
    high: number;
    low: number;
    close: number;
    volume: number;
  };

  onMount(async () => {
    const chart = createChart(container, {
      width: container.clientWidth,
      height: 650,
      layout: { background: { color: "#0b0e11" }, textColor: "#d1d4dc" },
      grid: { vertLines: { color: "#1f2933" }, horzLines: { color: "#1f2933" } },
      timeScale: { timeVisible: true, secondsVisible: false }
    });

    const candles = chart.addCandlestickSeries({
      upColor: "#26a69a",
      downColor: "#ef5350",
      borderUpColor: "#26a69a",
      borderDownColor: "#ef5350",
      wickUpColor: "#26a69a",
      wickDownColor: "#ef5350"
    });

    // 1) Snapshot DB (bars déjà closes)
    const res = await fetch(`${API}/bars/recent?symbol=${encodeURIComponent(symbol)}&timeframe=1m&limit=500`);
    const bars: Bar[] = await res.json();

    candles.setData(
      bars.map((b) => ({
        time: Math.floor(new Date(b.ts).getTime() / 1000),
        open: b.open,
        high: b.high,
        low: b.low,
        close: b.close
      }))
    );

    // 2) Live SSE (bar:update + bar:close)
    const es = new EventSource(`${API}/sse/bars-live?symbol=${encodeURIComponent(symbol)}`);

    const upsert = (b: Bar) => {
      candles.update({
        time: Math.floor(new Date(b.ts).getTime() / 1000),
        open: b.open,
        high: b.high,
        low: b.low,
        close: b.close
      });
    };

    es.addEventListener("bar:update", (e) => upsert(JSON.parse((e as MessageEvent).data)));
    es.addEventListener("bar:close", (e) => upsert(JSON.parse((e as MessageEvent).data)));

    const onResize = () => chart.applyOptions({ width: container.clientWidth });
    window.addEventListener("resize", onResize);

    return () => {
      window.removeEventListener("resize", onResize);
      es.close();
      chart.remove();
    };
  });
</script>

<div bind:this={container} style="width: 100%; height: 100vh;"></div>
