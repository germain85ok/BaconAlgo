import { rockets } from '$lib/stores/rockets';

export function startFakeRockets() {
  setInterval(() => {
    rockets.set([
      { symbol: 'AAPL', score: 87, state: 'ARMED' },
      { symbol: 'NVDA', score: 92, state: 'IGNITION' },
      { symbol: 'TSLA', score: 78, state: 'WATCH' }
    ]);
  }, 2000);
}
