import { writable } from 'svelte/store';

export const sentimentRaw = writable({
  SPX: { key: 'SPX', value: 'Bullish', change: 1.2, confidence: 78 },
  BTC: { key: 'BTC', value: 'Neutral', change: 0.1, confidence: 52 },
  DXY: { key: 'DXY', value: 'Bearish', change: -0.8, confidence: 65 }
});
