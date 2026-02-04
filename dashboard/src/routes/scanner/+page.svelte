<script lang="ts">
  import { onMount } from "svelte";

  const API = "http://127.0.0.1:3000";
  const timeframe = "1m";

  // Mets ta watchlist ici (tu pourras la rendre editable ensuite)
  let watchlist = ["AAPL", "MSFT", "NVDA", "TSLA"];

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

  type Row = {
    symbol: string;
    last?: number;
    open?: number;
    volume?: number;
    ts?: string;
    chgPct?: number;
    dir?: "up" | "down" | "flat";
  };

  let q = "";
  let rows: Record<string, Row> = {};

  const view = () => {
    const list = watchlist
      .filter((s) => s.includes(q.trim().toUpperCase()))
      .map((s) => rows[s] ?? { symbol: s });

    return list.sort((a, b) => Math.abs(b.chgPct ?? 0) - Math.abs(a.chgPct ?? 0));
  };

  const upsert = (b: Bar) => {
    const last = b.close;
    const open = b.open;
    const chgPct = open ? ((last - open) / open) * 100 : 0;

    const dir =
      chgPct > 0.0001 ? "up" :
      chgPct < -0.0001 ? "down" : "flat";

    rows = {
      ...rows,
      [b.symbol]: {
        symbol: b.symbol,
        last,
        open,
        volume: b.volume,
        ts: b.ts,
        chgPct,
        dir
      }
    };
  };

  onMount(() => {
    const symbols = encodeURIComponent(watchlist.join(","));
    const es = new EventSource(`${API}/sse/scanner?symbols=${symbols}&timeframe=${timeframe}`);

    es.addEventListener("bar:update", (e) => upsert(JSON.parse((e as MessageEvent).data)));
    es.addEventListener("bar:close", (e) => upsert(JSON.parse((e as MessageEvent).data)));

    return () => es.close();
  });
</script>

<div style="padding: 16px; color: #d1d4dc; background: #0b0e11; min-height: 100vh;">
  <div style="display:flex; gap:12px; align-items:center; margin-bottom:12px;">
    <h2 style="margin:0;">Scanner</h2>
    <input
      placeholder="Filter (ex: NV)"
      bind:value={q}
      style="background:#0f141a; border:1px solid #1f2933; color:#d1d4dc; padding:8px 10px; border-radius:8px; width:240px;"
    />
  </div>

  <table style="width:100%; border-collapse:collapse;">
    <thead>
      <tr style="text-align:left; border-bottom:1px solid #1f2933;">
        <th style="padding:10px 6px;">Symbol</th>
        <th style="padding:10px 6px;">Last</th>
        <th style="padding:10px 6px;">Chg % (bar)</th>
        <th style="padding:10px 6px;">Volume</th>
        <th style="padding:10px 6px;">Ts</th>
      </tr>
    </thead>
    <tbody>
      {#each view() as r (r.symbol)}
        <tr style="border-bottom:1px solid #141b22;">
          <td style="padding:10px 6px; font-weight:600;">{r.symbol}</td>
          <td style="padding:10px 6px;">{r.last?.toFixed(2) ?? "-"}</td>
          <td style="padding:10px 6px; color:
            {r.dir === "up" ? "#26a69a" : r.dir === "down" ? "#ef5350" : "#d1d4dc"};">
            {r.chgPct !== undefined ? `${r.chgPct.toFixed(3)}%` : "-"}
          </td>
          <td style="padding:10px 6px;">{r.volume?.toFixed(2) ?? "-"}</td>
          <td style="padding:10px 6px; opacity:0.7;">{r.ts ?? "-"}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <div style="margin-top:14px; opacity:0.7; font-size:12px;">
    Tip: ajoute ton routing vers /scanner dans ton menu (si tu en as un), sinon va direct sur /scanner.
  </div>
</div>
