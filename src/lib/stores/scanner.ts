import { writable } from 'svelte/store';

export const scannerResults = writable([
  { symbol: 'AAPL', setup: 'Breakout', score: 92 },
  { symbol: 'TSLA', setup: 'Momentum', score: 88 },
  { symbol: 'NVDA', setup: 'Reversal', score: 96 }
]);
