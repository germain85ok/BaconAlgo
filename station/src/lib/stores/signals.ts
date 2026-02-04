import { writable, derived } from 'svelte/store';

export interface Signal {
  id: string;
  symbol: string;
  horizon: string;
  direction: string;
  score: number;
  grade: string;
  entry: number;
  stop_loss: number;
  targets: number[];
  risk_reward: number;
  near_fvg: boolean;
  near_ob: boolean;
  near_golden_pocket: boolean;
  near_npoc: boolean;
  near_avwap: boolean;
  bos_confirmed: boolean;
  choch_detected: boolean;
  poc?: number;
  vah?: number;
  val?: number;
  wave_phase?: string;
  confluence_count: number;
  power_score: number;
  whale_score: number;
  whale_bars: number;
  absorption_count: number;
  created_at: string;
  expires_at?: string;
}

export const signals = writable<Map<string, Signal>>(new Map());

export const topSignals = derived(signals, ($signals) => 
  Array.from($signals.values())
    .sort((a, b) => b.score - a.score)
    .slice(0, 20)
);

export const legendarySignals = derived(signals, ($signals) =>
  Array.from($signals.values()).filter(s => s.grade === 'LEGENDARY')
);

export const epicSignals = derived(signals, ($signals) =>
  Array.from($signals.values()).filter(s => s.grade === 'EPIC')
);

export const stats = derived(signals, ($signals) => {
  const all = Array.from($signals.values());
  return {
    total: all.length,
    legendary: all.filter(s => s.score >= 90).length,
    epic: all.filter(s => s.score >= 80 && s.score < 90).length,
    rare: all.filter(s => s.score >= 70 && s.score < 80).length,
    avgScore: all.length > 0 ? all.reduce((sum, s) => sum + s.score, 0) / all.length : 0,
    whaleSignals: all.filter(s => s.whale_score > 70).length,
    avgConfluence: all.length > 0 ? all.reduce((sum, s) => sum + s.confluence_count, 0) / all.length : 0,
    avgPower: all.length > 0 ? all.reduce((sum, s) => sum + s.power_score, 0) / all.length : 0,
  };
});

export function connectSignalStream(url: string = '/signals/live') {
  const es = new EventSource(url);

  es.onmessage = (e) => {
    try {
      const signal: Signal = JSON.parse(e.data);
      
      signals.update(map => {
        const newMap = new Map(map);
        newMap.set(signal.id, signal); // Use signal.id as key to preserve all signals
        return newMap;
      });
    } catch (err) {
      console.error('Failed to parse signal:', err);
    }
  };

  es.onerror = (err) => {
    console.error('SSE error:', err);
  };

  return () => es.close();
}

export const filters = writable({
  minScore: 0,
  minWhale: 0,
  minConfluence: 0,
  minPower: 0,
  setups: {
    fvg: false,
    ob: false,
    goldenPocket: false,
    bos: false,
    choch: false,
    npoc: false,
    avwap: false,
  }
});

export const filteredSignals = derived(
  [signals, filters],
  ([$signals, $filters]) => {
    return Array.from($signals.values()).filter(signal => {
      if (signal.score < $filters.minScore) return false;
      if (signal.whale_score < $filters.minWhale) return false;
      if (signal.confluence_count < $filters.minConfluence) return false;
      if (signal.power_score < $filters.minPower) return false;

      if ($filters.setups.fvg && !signal.near_fvg) return false;
      if ($filters.setups.ob && !signal.near_ob) return false;
      if ($filters.setups.goldenPocket && !signal.near_golden_pocket) return false;
      if ($filters.setups.bos && !signal.bos_confirmed) return false;
      if ($filters.setups.choch && !signal.choch_detected) return false;
      if ($filters.setups.npoc && !signal.near_npoc) return false;
      if ($filters.setups.avwap && !signal.near_avwap) return false;

      return true;
    }).sort((a, b) => b.score - a.score);
  }
);
