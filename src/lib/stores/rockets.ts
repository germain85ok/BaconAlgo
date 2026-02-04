import { writable } from 'svelte/store';

export const rockets = writable([
  { symbol: 'NVDA', score: 96, state: 'IGNITION' },
  { symbol: 'META', score: 91, state: 'ARMED' }
]);
