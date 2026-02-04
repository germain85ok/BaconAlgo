import { browser } from '$app/environment';
import { sentimentRaw } from '$lib/stores/sentiment';

export function connectStream() {
  if (!browser) return; // ⛔ STOP SSR

  const ws = new WebSocket('ws://localhost:3000');

  ws.onopen = () => {
    console.log('🟢 Stream connecté');
  };

  ws.onclose = () => {
    console.log('🔴 Stream fermé');
  };

  ws.onerror = (e) => {
    console.error('❌ Stream erreur', e);
  };

  ws.onmessage = (e) => {
    const msg = JSON.parse(e.data);

    if (msg.type === 'snapshot') {
      sentimentRaw.set(msg.payload);
    }

    if (msg.type === 'sentiment.update') {
      sentimentRaw.update((s) => ({
        ...s,
        [msg.payload.key]: msg.payload
      }));
    }
  };
}
